export default class ListRoutesUseCase {
  constructor(gatewasmService) {
    this.gatewasmService = gatewasmService;
  }

  async execute() {
    return this.gatewasmService.getRoutes();
  }
}
