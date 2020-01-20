import React, { useState, useEffect, useCallback, Fragment } from "react";
import styled from "styled-components";
import { Tabs, Tab, Tree, Button } from "@react95/core";

import TreeArea from "../TreeArea/TreeArea";
import TriggeredModal from "../TriggeredModal/TriggeredModal";
import CreateRouteModal from "../CreateRouteModal/CreateRouteModal";
import CreateRouteUseCase from "../../domain/useCases/CreateRouteUseCase";
import ListRoutesUseCase from "../../domain/useCases/ListRoutesUseCase";
import Loader from "../Loader/Loader";
import Container from "../Container/Container";

function groupBy(data, fn) {
  return data.reduce((obj, value) => {
    var key = fn(value);

    if (!obj.hasOwnProperty(key)) {
      obj[key] = [];
    }

    obj[key].push(value);

    return obj;
  }, {});
}

function Main({ gatewasmService }) {
  const [routes, setRoutes] = useState({});

  const listRoutes = useCallback(async () => {
    setRoutes({
      loading: true
    });

    try {
      const data = await new ListRoutesUseCase(gatewasmService).execute();

      const grouped = groupBy(data, route => route.path);
      const routes = Object.entries(grouped).map(([path, routes], id) => ({
        id,
        label: path,
        children: routes.map((route, id) => ({
          id,
          label: route.method,
          iconName: "bat_exec"
        }))
      }));

      setRoutes({
        loading: false,
        data: routes
      });
    } catch (error) {
      console.error(error);
      setRoutes({
        loading: false,
        error
      });
    }
  }, [gatewasmService, setRoutes]);

  useEffect(() => {
    listRoutes();
  }, [listRoutes]);

  const createRoute = async data =>
    new CreateRouteUseCase(gatewasmService).execute(data);

  return (
    <Fragment>
      <Loader
        modalTitle="Initialization"
        loading={routes.loading}
        error={routes.error}
        onRetry={listRoutes}
      />
      {routes.data && (
        <Container>
          <Tabs>
            <Tab title="Configuration">
              <TreeArea>{<Tree data={routes.data} />}</TreeArea>

              <TriggeredModal
                trigger={openModal => (
                  <Button onClick={openModal}>New Route</Button>
                )}
                modal={closeModal => (
                  <CreateRouteModal
                    onSubmit={data =>
                      createRoute(data)
                        .then(listRoutes)
                        .then(closeModal)
                    }
                    onClose={closeModal}
                  />
                )}
              />
            </Tab>
          </Tabs>
        </Container>
      )}
    </Fragment>
  );
}

export default Main;
