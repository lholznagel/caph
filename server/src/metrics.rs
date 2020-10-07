use rusoto_cloudwatch::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Metrics(CloudWatchClient);

impl Metrics {
    const METRIC_MARKET_CACHE_COUNT: &'static str = "market_cache_count";
    const METRIC_MARKET_CACHE_TIME: &'static str = "market_cache_time";

    const METRIC_SDE_ITEM_CACHE_COUNT: &'static str = "sde_item_cache_count";
    const METRIC_SDE_CACHE_TIME: &'static str = "sde_cache_time";

    pub async fn put_market_metrics(&self, order_count: usize, time: u128) {
        let mut metric_count = MetricDatum::default();
        metric_count.metric_name = Self::METRIC_MARKET_CACHE_COUNT.into();
        metric_count.value = Some(order_count as f64);

        let mut metric_time = MetricDatum::default();
        metric_time.metric_name = Self::METRIC_MARKET_CACHE_TIME.into();
        metric_time.value = Some(time as f64);

        self.send(vec![metric_count, metric_time]).await;
    }

    pub async fn put_sde_metris(&self, item_count: usize, time: u128) {
        let mut metric_items_count = MetricDatum::default();
        metric_items_count.metric_name = Self::METRIC_SDE_ITEM_CACHE_COUNT.into();
        metric_items_count.value = Some(item_count as f64);

        let mut metric_time = MetricDatum::default();
        metric_time.metric_name = Self::METRIC_SDE_CACHE_TIME.into();
        metric_time.value = Some(time as f64);

        self.send(vec![metric_items_count, metric_time]).await;
    }

    /// Sends the metric to cloudwatch, will silently fail on error
    async fn send(&self, metric_data: Vec<MetricDatum>) {
        let metric_data = PutMetricDataInput {
            namespace: "eve_server".into(),
            metric_data,
        };

        match async_std::future::timeout(
            Duration::from_secs(10),
            self.0.put_metric_data(metric_data),
        )
        .await
        {
            Ok(Ok(())) => log::debug!("Send metric to cloudwatch"),
            Ok(Err(e)) => log::error!("Error sending metrics to cloudwatch: {}.", e),
            Err(_) => log::error!("Timedout sendong metrics to cloudwatch."),
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self(CloudWatchClient::new(Default::default()))
    }
}
