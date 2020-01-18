import React from "react";
import styled from "styled-components";
import { Dropdown as React95Dropdown } from "@react95/core";

const Wrapper = styled.div`
  display: flex;

  > :first-child {
    flex-grow: 1;
    height: auto;

    > select {
      height: auto;
    }

    &::after {
      width: 18px;
      height: 19px;
      right: 0px;
      top: 3px;
    }
  }
`;

function Dropdown(props) {
  return (
    <Wrapper>
      <React95Dropdown {...props} />
    </Wrapper>
  );
}

export default Dropdown;
