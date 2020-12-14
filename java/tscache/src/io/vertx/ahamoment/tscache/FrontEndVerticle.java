package io.vertx.ahamoment.tscache;

import io.vertx.core.AbstractVerticle;
import io.vertx.core.Promise;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.http.HttpServer;

/**
 * Created by zhongyangwu on 12/13/20.
 */
public class FrontEndVerticle extends AbstractVerticle {
    EventBus eventBus;

    public void start(Promise<Void> startFuture) {
        eventBus = vertx.eventBus();
        HttpServer server = vertx.createHttpServer().requestHandler(req ->
                eventBus.request("joke", "",
                        res -> req.response()
                                .putHeader("Content-Type", "text/plain")
                                .end(res.result().body().toString())));

        server.listen(8080, res -> startFuture.complete());
    }
}
