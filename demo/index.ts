import { Hono } from 'hono';
import { Redis } from 'ioredis';

const app = new Hono();

const appState = {
    token: "",
    name: "demo-client".split(' ').join('').toLocaleLowerCase(),
};

(async () => {
    const init_token = await fetch(process.env.M2M_AUTH_REGISTRY_BASE_URL + '/node/register', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'x-register-token': process.env.REGISTRATION_TOKEN!,
        },
        body: JSON.stringify({ app_node: appState.name })
    }).then(res => res.json());

    appState.token = init_token.initial_token;
})();

const redis = new Redis({
    host: process.env.REDIS_HOST!,
    port: Number(process.env.REDIS_PORT!),
});

const redis2 = new Redis({
    host: process.env.REDIS_HOST!,
    port: Number(process.env.REDIS_PORT!),
});

redis.subscribe('m2m:auth:ping:' + appState.name, (err, count) => {
    if (err) {
        console.error(err);
    }
    console.log(`Subscribed to m2m:auth:ping:${appState.name} channel.`);
});

redis.subscribe('m2m:auth:grant_token:' + appState.name, (err, count) => {
    if (err) {
        console.error(err);
    }
    console.log(`Subscribed to m2m:auth:grant_token:${appState.name} channel.`);
});


redis.on('message', (channel, message) => {
    if (channel === 'm2m:auth:ping:' + appState.name) {
        redis2.publish('m2m:auth:mark_attendance', appState.name);

        console.log('Ping received');
    }
    if (channel === 'm2m:auth:grant_token:' + appState.name) {
        appState.token = message;

        console.log('Token updated');
    }
});

app.get('/config', async (ctx) => {
    return ctx.json(appState);
});

export default app;