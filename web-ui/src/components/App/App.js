import React from "react";

import Main from "../Main/Main";
import GatewasmHttpService from "../../domain/services/GatewasmHttpService";

function App() {
  const gatewasmService = new GatewasmHttpService();

  return <Main gatewasmService={gatewasmService} />;
}

export default App;
