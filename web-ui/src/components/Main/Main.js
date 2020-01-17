import React from "react";
import styled from "styled-components";
import { Tabs, Tab, Tree, Button, Modal, List } from "@react95/core";

const Container = styled.div`
  margin: 8px;
`;

const TreeArea = styled.div`
  background: #ffffff;
  border-right-width: initial;
  border-bottom-width: initial;
  border-right-color: initial;
  border-bottom-color: initial;
  box-shadow: rgb(195, 199, 203) -1px -1px 0px 0px inset,
    rgb(0, 0, 0) 1px 1px 0px 0px inset, rgb(255, 255, 255) 0.5px 0.5px 0px 0.5px;
  border-style: solid none none solid;
  border-image: initial;
  border-left: 1px solid rgb(134, 138, 142);
  border-top: 1px solid rgb(134, 138, 142);
  outline: none;
  padding: 0 8px;
  margin-bottom: 8px;

  > ul {
    padding-top: 4px;
    padding-bottom: 4px;
  }
`;

function Main() {
  const data = [
    {
      id: 0,
      label: "/yell",
      children: [
        {
          id: 0,
          label: "GET",
          iconName: "bat_exec"
        },
        {
          id: 1,
          label: "POST"
        }
      ]
    },
    {
      id: 0,
      label: "/do-something",
      children: [
        {
          id: 0,
          label: "GET"
        },
        {
          id: 1,
          label: "POST"
        }
      ]
    },
    {
      id: 0,
      label: "/delete",
      children: [
        {
          id: 0,
          label: "GET"
        },
        {
          id: 1,
          label: "POST"
        }
      ]
    }
  ];

  return (
    <Container>
      <Tabs>
        <Tab title="Configuration">
          <TreeArea>
            <Tree data={data} />
          </TreeArea>
          <Button>New Route</Button>
        </Tab>
      </Tabs>
      <Modal
        icon="bat_exec"
        title="New Route"
        buttons={[{ value: "Ok" }, { value: "Cancel" }]}
        menu={[
          {
            name: "File",
            list: (
              <List>
                <List.Item>Exit</List.Item>
              </List>
            )
          },
          {
            name: "Help",
            list: (
              <List>
                <List.Item>About</List.Item>
              </List>
            )
          }
        ]}
      >
        Hello
      </Modal>
    </Container>
  );
}

export default Main;
