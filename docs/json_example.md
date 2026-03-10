```json
{
  "process": "LoanApproval",
  "version": "2.1.0",
  "initial_state": "Draft",
  "transitions": [
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path A: Auto-Approve small loans for premium members",
      "to": "Approved",
      "priority": 10,
      "guards": [
        { "type": "LessThan", "path": "entity.Amount", "value": 1000 },
        { "type": "IsEqual", "path": "ext.MemberTier", "value": "Premium" }
      ],
      "effects": ["NotifyCustomer", "TriggerPayout"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path B: Escalate large loans or high-risk cases",
      "to": "PendingVP",
      "priority": 5,
      "guards": [
        { "type": "GreaterThan", "path": "entity.Amount", "value": 50000 }
      ],
      "effects": ["EmailVP", "LogHighValueRisk"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path C: Default path for standard review",
      "to": "PendingManager",
      "priority": 1,
      "guards": [], 
      "effects": ["AssignToQueue"]
    },
    {
      "from": "PendingManager",
      "action": "Approve",
      "comment": "Temporal Guard: Requires a FraudCheck event in the last 24h",
      "to": "Approved",
      "guards": [
        { 
          "type": "OccurredWithin", 
          "target_action": "FraudCheck", 
          "window_seconds": 86400 
        }
      ]
    }
  ]
}
```