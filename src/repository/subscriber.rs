use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static!{

    static ref SUBSCRIBERS: DashMap<String, Dashmap<String, Subscriber>> = DashMap::new();

}

pub struct SubscriberRepository;

imp SubscriberRepository {

}