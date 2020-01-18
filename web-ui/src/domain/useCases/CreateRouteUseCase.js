export default class CreateRouteUseCase {
  constructor(gatewasmService) {
    this.gatewasmService = gatewasmService;
  }

  async execute({ path, method, handler }) {
    return this.gatewasmService.createRoute({ path, method, handler });
  }
}
