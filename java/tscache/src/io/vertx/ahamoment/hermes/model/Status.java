package io.vertx.ahamoment.hermes.model;

import io.vertx.core.eventbus.MessageCodec;

/**
 * Created by zhongyangwu on 3/21/21.
 */
public class Status {
    boolean online;
    int userId;

    public int getUserId() {
        return userId;
    }

    public boolean isOnline(){
        return online;
    }
}
