export default class GatewasmHttpService {
  baseUrl = "http://eyrie.bazk.me:8900";

  async getRoutes() {
    const response = await fetch("http://eyrie.bazk.me:8900/routes", {
      method: "GET",
      mode: "cors",
      cache: "no-cache",
      referrerPolicy: "no-referrer"
    });

    return response.json();
  }

  async createRoute({ path, method, handler }) {
    const buffer = await handler.arrayBuffer();
    const code = window.btoa(buffer);

    const response = await fetch("http://eyrie.bazk.me:8900/routes", {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      headers: {
        "Content-Type": "application/json"
      },
      referrerPolicy: "no-referrer",
      body: JSON.stringify({
        path,
        method,
        handler: { code }
      })
    });

    return response.json();
  }
}
