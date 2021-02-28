# Github API & ecosystem

## Webhook

Webhook refers to the requests sent by github when certain events happens. For example, PR merge, PR opened. 

For a detailed description, find it [here](https://docs.github.com/en/developers/webhooks-and-events/webhook-events-and-payloads)

### Tasks

Here we list a few webhooks that may needed to complete certain tasks.
1. **Able to merge**? To check if PR is able to merge, one could choose to use **Status** event. This event triggered when the PR checks completed. Users can then check if the PR is approved by PR API.
