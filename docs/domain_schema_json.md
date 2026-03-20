```json
{
  "domain": "OrderManagement",
  "version": "1.0.0",
  "fields": [
    {
      "name": "order_total",
      "type": "Number",
      "description": "The final amount charged to the customer"
    },
    {
      "name": "currency",
      "type": "String",
      "description": "ISO 4217 currency code"
    },
    {
      "name": "is_priority",
      "type": "Boolean"
    },
    {
      "name": "customer_tier",
      "type": "String",
      "enum": ["Gold", "Silver", "Bronze"]
    }
  ]
}
.
```

```json
{
  "domain": "SubscriptionService",
  "version": "1.0.0",
  "fields": [
    { "name": "subscription_end", "type": "DateTime" },
    { "name": "grace_period_seconds", "type": "Duration" },
    { "name": "is_active", "type": "Boolean" }
  ]
}
```