use std::env;
use std::fs::File;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::{IntoRequest, Request};

use crate::authorize::{ApplicationCredentials, TokenManager, TLS_CERTS};
use crate::pubsub::api;
use crate::pubsub::api::publisher_client::PublisherClient;
use crate::pubsub::api::subscriber_client::SubscriberClient;
use crate::pubsub::{Error, Subscription, Topic, TopicConfig};

/// The Pub/Sub client, tied to a specific project.
#[derive(Clone)]
pub struct Client {
    pub(crate) project_name: String,
    pub(crate) publisher: PublisherClient<Channel>,
    pub(crate) subscriber: SubscriberClient<Channel>,
    pub(crate) token_manager: Arc<Mutex<TokenManager>>,
}

impl Client {
    const DOMAIN_NAME: &'static str = "pubsub.googleapis.com";
    const ENDPOINT: &'static str = "https://pubsub.googleapis.com";
    const SCOPES: [&'static str; 2] = [
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/pubsub",
    ];

    pub(crate) async fn construct_request<T: IntoRequest<T>>(
        &mut self,
        request: T,
    ) -> Result<Request<T>, Error> {
        let mut request = request.into_request();
        let token = self.token_manager.lock().await.token().await?;
        let metadata = request.metadata_mut();
        metadata.insert("authorization", token.parse().unwrap());
        Ok(request)
    }

    /// Create a new client for the specified project.
    ///
    /// Credentials are looked up in the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
    pub async fn new(project_name: impl Into<String>) -> Result<Client, Error> {
        if let Ok(host) = env::var("PUBSUB_EMULATOR_HOST") {
            return Client::insecure(project_name, host).await
        }

        let path = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
        let file = File::open(path)?;
        let creds = json::from_reader(file)?;

        Client::from_credentials(project_name, creds).await
    }

    async fn insecure(
        project_name: impl Into<String>,
        domain_name: String,
    ) -> Result<Client, Error> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(TLS_CERTS))
            .domain_name(&domain_name);

        let channel = Channel::from_shared(format!("http://{}", domain_name))?
            .tls_config(tls_config)?
            .connect()
            .await?;

        Ok(Client {
            project_name: project_name.into(),
            publisher: PublisherClient::new(channel.clone()),
            subscriber: SubscriberClient::new(channel),
            token_manager: Arc::new(Mutex::new(TokenManager::Insecure)),
        })
    }

    /// Create a new client for the specified project with custom credentials.
    /// Allows creation of client without credentials, should not be called directly
    async fn from_credentials(
        project_name: impl Into<String>,
        creds: ApplicationCredentials,
    ) -> Result<Client, Error> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(TLS_CERTS))
            .domain_name(Client::DOMAIN_NAME);

        let channel = Channel::from_static(Client::ENDPOINT)
            .tls_config(tls_config)?
            .connect()
            .await?;

        Ok(Client {
            project_name: project_name.into(),
            publisher: PublisherClient::new(channel.clone()),
            subscriber: SubscriberClient::new(channel),
            token_manager: Arc::new(Mutex::new(TokenManager::new(
                creds,
                Client::SCOPES.as_ref(),
            ))),
        })
    }

    /// Create a new topic.
    pub async fn create_topic(
        &mut self,
        topic_id: &str,
        config: TopicConfig,
    ) -> Result<Topic, Error> {
        let request = api::Topic {
            name: format!(
                "projects/{0}/topics/{1}",
                self.project_name.as_str(),
                topic_id,
            ),
            labels: config.labels,
            message_storage_policy: None,
            kms_key_name: String::new(),
        };
        let request = self.construct_request(request).await?;
        let response = self.publisher.create_topic(request).await?;
        let topic = response.into_inner();

        Ok(Topic::new(self.clone(), topic.name))
    }

    /// List all exisiting topics.
    pub async fn topics(&mut self) -> Result<Vec<Topic>, Error> {
        let mut topics = Vec::new();
        let page_size = 25;
        let mut page_token = String::default();

        loop {
            let request = api::ListTopicsRequest {
                project: format!("projects/{0}", self.project_name.as_str()),
                page_size,
                page_token,
            };
            let request = self.construct_request(request).await?;
            let response = self.publisher.list_topics(request).await?;
            let response = response.into_inner();
            page_token = response.next_page_token;
            topics.extend(
                response
                    .topics
                    .into_iter()
                    .map(|topic| Topic::new(self.clone(), topic.name)),
            );
            if page_token.is_empty() {
                break;
            }
        }

        Ok(topics)
    }

    /// Get a handle to a specific topic.
    pub async fn topic(&mut self, id: &str) -> Result<Option<Topic>, Error> {
        let request = api::GetTopicRequest {
            topic: format!("projects/{0}/topics/{1}", self.project_name.as_str(), id),
        };
        let request = self.construct_request(request).await?;
        let response = self.publisher.get_topic(request).await?;
        let topic = response.into_inner();

        Ok(Some(Topic::new(self.clone(), topic.name)))
    }

    /// List all existing subscriptions (to any topic).
    pub async fn subscriptions(&mut self) -> Result<Vec<Subscription>, Error> {
        let mut subscriptions = Vec::new();
        let page_size = 25;
        let mut page_token = String::default();

        loop {
            let request = api::ListSubscriptionsRequest {
                project: format!("projects/{0}", self.project_name.as_str()),
                page_size,
                page_token,
            };
            let request = self.construct_request(request).await?;
            let response = self.subscriber.list_subscriptions(request).await?;
            let response = response.into_inner();
            page_token = response.next_page_token;
            subscriptions.extend(
                response
                    .subscriptions
                    .into_iter()
                    .map(|subscription| Subscription::new(self.clone(), subscription.name)),
            );
            if page_token.is_empty() {
                break;
            }
        }

        Ok(subscriptions)
    }

    /// Get a handle of a specific subscription.
    pub async fn subscription(&mut self, id: &str) -> Result<Option<Subscription>, Error> {
        let request = api::GetSubscriptionRequest {
            subscription: format!(
                "projects/{0}/subscriptions/{1}",
                self.project_name.as_str(),
                id,
            ),
        };
        let request = self.construct_request(request).await?;
        let response = self.subscriber.get_subscription(request).await?;
        let subscription = response.into_inner();

        Ok(Some(Subscription::new(self.clone(), subscription.name)))
    }
}
