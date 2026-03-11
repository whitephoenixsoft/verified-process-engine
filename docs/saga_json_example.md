```json
{
  "name": "Pending_Payment",
  "is_saga": true, 
  "transitions": [
    { "action": "PAYMENT_CONFIRMED", "to": "Approved" },
    { "action": "PAYMENT_FAILED", "to": "Draft" },
    { 
      "action": "AUTO_TICK", 
      "to": "Draft",
      "guards": [ { "type": "TimeElapsed", "seconds": 3600 } ] 
    }
  ]
}

```

```json
{
  "name": "Pending_Payment_Gateway",
  "is_transient": true,
  "transitions": [
    {
      "action": "GATEWAY_SUCCESS",
      "to": "Paid"
    },
    {
      "action": "GATEWAY_FAILURE",
      "to": "Payment_Error"
    },
    {
      "action": "AUTO_TICK",
      "to": "Draft",
      "guards": [
        { 
          "type": "TimeElapsed", 
          "seconds": 300, 
          "since": "sys.last_transition_time" 
        }
      ]
    }
  ]
}

```