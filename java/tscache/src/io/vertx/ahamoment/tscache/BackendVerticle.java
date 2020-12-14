package io.vertx.ahamoment.tscache;

import io.vertx.core.AbstractVerticle;
import io.vertx.core.Vertx;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.eventbus.Message;
import io.vertx.core.json.JsonObject;
import io.vertx.ext.web.client.HttpRequest;
import io.vertx.ext.web.client.WebClient;
import io.vertx.ext.web.client.predicate.ResponsePredicate;
import io.vertx.ext.web.codec.BodyCodec;

public class BackendVerticle extends AbstractVerticle {

    private HttpRequest<JsonObject> request;
    private EventBus eventBus;

    @Override
    public void start() {
        eventBus = vertx.eventBus();
        request = WebClient.create(vertx) // (1)
                .get(443, "icanhazdadjoke.com", "/") // (2)
                .ssl(true)  // (3)
                .putHeader("Accept", "application/json")  // (4)
                .as(BodyCodec.jsonObject()) // (5)
                .expect(ResponsePredicate.SC_OK);  // (6)
        eventBus.consumer("joke", this::fetchJoke);
    }

    private void fetchJoke(Message<Object> msg) {
        request.send(asyncResult -> {
            if (asyncResult.succeeded()) {
                msg.reply(asyncResult.result().body().getString("joke"));
            }
        });
    }
}
