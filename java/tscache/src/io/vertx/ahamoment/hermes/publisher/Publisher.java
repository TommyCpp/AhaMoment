package io.vertx.ahamoment.hermes.publisher;


import io.vertx.ahamoment.hermes.Address;
import io.vertx.ahamoment.hermes.model.Status;
import io.vertx.core.AbstractVerticle;
import io.vertx.core.Future;
import io.vertx.core.Handler;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.http.HttpServer;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.json.Json;

/**
 * Created by zhongyangwu on 3/21/21.
 */
public class Publisher extends AbstractVerticle {

    private final Handler<HttpServerRequest> handler;
    private EventBus eventBus;

    public Publisher() {
        this.handler = request -> {
            Status status = Json.decodeValue(request.toString(), Status.class);
            this.publishToDispatch(Json.encode(status));
        };
    }

    public Publisher(Handler<HttpServerRequest> handler) {
        this.handler = handler;
    }

    @Override
    public void start(Future<Void> startFuture) throws Exception {
        this.eventBus = vertx.eventBus();
        HttpServer server = vertx.createHttpServer().requestHandler(this.handler);
        server.listen(8081);

    }

    private void publishToDispatch(String msg) {
        this.eventBus.request(Address.DISPATCHER, msg, resp -> {
            // discard for now
        });
    }
}
