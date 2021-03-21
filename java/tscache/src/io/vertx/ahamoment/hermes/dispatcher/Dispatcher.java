package io.vertx.ahamoment.hermes.dispatcher;

import io.vertx.ahamoment.hermes.Address;
import io.vertx.ahamoment.hermes.model.Status;
import io.vertx.core.AbstractVerticle;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.eventbus.Message;
import io.vertx.core.json.Json;

import java.util.HashMap;
import java.util.Set;

/**
 * Created by zhongyangwu on 3/21/21.
 */
public class Dispatcher extends AbstractVerticle {
    EventBus eventBus;
    HashMap<Integer, Set<Integer>> subscriptions;

    public Dispatcher(){
        this.subscriptions = new HashMap<>();
    }

    @Override
    public void start() throws Exception {
        this.eventBus = vertx.eventBus();

        this.eventBus.consumer(Address.DISPATCHER, this::handle);
    }

    private void handle(Message<String> msg) {
        //TODO: figure out how to register the subscriber
        Status status = Json.decodeValue(msg.body(), Status.class);
        for(Integer id: this.subscriptions.getOrDefault(status.getUserId(), null)){
            this.eventBus.request(getSubscriber(id), msg, resp -> {
                if(resp.failed()){
                    // if cannot deliver, then delete the subscriber from the subscription set.
                    this.subscriptions.get(status.getUserId()).remove(id);
                }
            });
        }
    }

    private String getSubscriber(Integer id){
        return Address.SUBSCRIBER + "." + id;
    }
}
