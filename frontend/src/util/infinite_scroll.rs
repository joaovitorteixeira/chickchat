use leptos::prelude::*;

pub type HasMoreTuple = (ReadSignal<bool>,  WriteSignal<bool>); 
pub type MessagesTuple<T> = (ReadSignal<Vec<T>>,WriteSignal<Vec<T>> );


pub struct InfiniteScroll<T> {
    pub has_more_tuple: HasMoreTuple,
    pub messages_tuple: MessagesTuple<T>,
}

impl <T> InfiniteScroll<T> 
where T: Clone + Send + Sync + 'static
{
    pub fn new(
        has_more_tuple: HasMoreTuple,
        messages_tuple: MessagesTuple<T>,
    ) -> Self {
        Self {
            has_more_tuple,
            messages_tuple,
        }
    }

    pub async fn get_old_items<F, Fut> (
        &self,
        get_last_item_id: impl Fn() -> Option<String>,
        fetch_items: F,
    )     where 
    F: Fn(u8, Option<String>) -> Fut,
    Fut: std::future::Future<Output = Vec<T>>,
    {
        let (has_more, set_has_more) = &self.has_more_tuple;
        let (_, set_items) = &self.messages_tuple;
        
        if has_more.read_untracked() == false {
            return;
        }
        let old_items = fetch_items(10, get_last_item_id()).await;
        
        set_has_more.set(!old_items.is_empty());
        set_items.write().extend(old_items);
    }
}
