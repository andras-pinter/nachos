export class NachosClient {
    client;
    tunnel;

    constructor(tunnelURL) {
        this.tunnel = this.getTunnel(tunnelURL);
        this.client = this.getClient(this.tunnel);
        document.body.appendChild(this.client.getDisplay().getElement());
    }

    isConnected() {
        return this.tunnel.isConnected();
    }

    connect() {
        this.client.connect();
        setTimeout(() => {
            console.log(this.isConnected());
        }, 1000);
    }

    getClient(tunnel) {
        const client = new Guacamole.Client(tunnel);

        client.onargv = function (stream, mimetype, name) {
            if (mimetype !== 'text/plain')
                return;

            let reader = new Guacamole.StringReader(stream);
            let value = '';

            reader.ontext = function (text) {
                value += text;
            }

            reader.onend = function () {
                let stream = client.createArgumentValueStream('text/plain', name);
                let writer = new Guacamole.StringWriter(stream);
                writer.sendText(value);
            }
        }

        client.onstatechange = function (state) {
            console.log("State: " + state);
        }

        client.onerror = function (status) {
            console.log("Error: " + status);
            client.disconnect();
        }

        return client;
    }

    getTunnel(tunnelURL) {
        const tunnel = new Guacamole.WebSocketTunnel(tunnelURL);

        tunnel.onuuid = function (uuid) {
            console.log(uuid);
        }

        return tunnel;
    }
}
