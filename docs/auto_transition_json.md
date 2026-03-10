```json
{
  "domain": "OrderManagement",
  "version": "1.1.0",
  "initial_state": "Draft",
  "states": [
    {
      "name": "Draft",
      "transitions": [
        {
          "action": "Submit", 
          "to": "Validation_Point",
          "guards": [] 
        }
      ]
    },
    {
      "name": "Validation_Point",
      "transitions": [
        {
          "action": null, 
          "to": "VIP_Review",
          "guards": [
            { "type": "GreaterThan", "path": "rec.order_total", "value": 10000 }
          ]
        },
        {
          "action": null,
          "to": "Standard_Processing",
          "guards": [
            { "type": "Default" } 
          ]
        }
      ]
    },
    { "name": "VIP_Review" },
    { "name": "Standard_Processing" }
  ]
}

```