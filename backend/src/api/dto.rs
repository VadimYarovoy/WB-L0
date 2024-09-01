use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Order {
    #[validate(length(min = 4))]
    #[validate(required)]
    order_uid: Option<String>,
    #[validate(length(min = 4))]
    #[validate(required)]
    track_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery: Option<Delivery>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    payment: Option<Payment>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<Item>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shardkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sm_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oof_shard: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
struct Delivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_dt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_cost: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    goods_total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fee: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    chrt_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sale: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nm_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_order_validation() {
        let order = Order {
            order_uid: Some("1234".to_string()),
            track_number: Some("5678".to_string()),
            entry: Some("entry".to_string()),
            delivery: Some(Delivery {
                name: Some("John Doe".to_string()),
                phone: Some("1234567890".to_string()),
                zip: Some("12345".to_string()),
                city: Some("City".to_string()),
                address: Some("123 Street".to_string()),
                region: Some("Region".to_string()),
                email: Some("john.doe@example.com".to_string()),
            }),
            payment: Some(Payment {
                transaction: Some("trans123".to_string()),
                request_id: Some("req456".to_string()),
                currency: Some("USD".to_string()),
                provider: Some("Provider".to_string()),
                amount: Some(100),
                payment_dt: Some(1234567890),
                bank: Some("Bank".to_string()),
                delivery_cost: Some(10),
                goods_total: Some(90),
                custom_fee: Some(5),
            }),
            items: Some(vec![Item {
                chrt_id: Some(1),
                track_number: Some("item123".to_string()),
                price: Some(50),
                rid: Some("rid456".to_string()),
                name: Some("Item Name".to_string()),
                sale: Some(10),
                size: Some("M".to_string()),
                total_price: Some(60),
                nm_id: Some(2),
                brand: Some("Brand".to_string()),
                status: Some(1),
            }]),
            locale: Some("en_US".to_string()),
            internal_signature: Some("signature123".to_string()),
            customer_id: Some("customer123".to_string()),
            delivery_service: Some("Service".to_string()),
            shardkey: Some("shardkey123".to_string()),
            sm_id: Some(123),
            date_created: Some("2022-01-01".to_string()),
            oof_shard: Some("oof123".to_string()),
        };

        assert!(order.validate().is_ok());
    }

    #[test]
    fn test_order_validation_with_error() {
        let order = Order {
            order_uid: Some("123".to_string()), // incorrect len
            track_number: Some("5678".to_string()),
            entry: Some("entry".to_string()),
            delivery: Some(Delivery {
                name: Some("John Doe".to_string()),
                phone: Some("1234567890".to_string()),
                zip: Some("12345".to_string()),
                city: Some("City".to_string()),
                address: Some("123 Street".to_string()),
                region: Some("Region".to_string()),
                email: Some("john.doe@example.com".to_string()),
            }),
            payment: Some(Payment {
                transaction: Some("trans123".to_string()),
                request_id: Some("req456".to_string()),
                currency: Some("USD".to_string()),
                provider: Some("Provider".to_string()),
                amount: Some(100),
                payment_dt: Some(1234567890),
                bank: Some("Bank".to_string()),
                delivery_cost: Some(10),
                goods_total: Some(90),
                custom_fee: Some(5),
            }),
            items: Some(vec![Item {
                chrt_id: Some(1),
                track_number: Some("item123".to_string()),
                price: Some(50),
                rid: Some("rid456".to_string()),
                name: Some("Item Name".to_string()),
                sale: Some(10),
                size: Some("M".to_string()),
                total_price: Some(60),
                nm_id: Some(2),
                brand: Some("Brand".to_string()),
                status: Some(1),
            }]),
            locale: Some("en_US".to_string()),
            internal_signature: Some("signature123".to_string()),
            customer_id: Some("customer123".to_string()),
            delivery_service: Some("Service".to_string()),
            shardkey: Some("shardkey123".to_string()),
            sm_id: Some(123),
            date_created: Some("2022-01-01".to_string()),
            oof_shard: Some("oof123".to_string()),
        };

        assert_eq!(order.validate().is_ok(), false);
    }
}
