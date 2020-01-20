import React from "react";
import styled from "styled-components";
import PropTypes from "prop-types";

const Container = styled.div`
  height: 20px;
  position: relative;
  text-align: center;
  line-height: 20px;
  border-left: 1px solid #868a8e;
  border-top: 1px solid #868a8e;
  background: #fff;
  color: #000;
  box-shadow: inset -1px -1px 0 0 #c3c7cb, inset 1px 1px 0 0 #000000,
    0.5px 0.5px 0 0.5px #ffffff;
`;

const Inner = styled.div.attrs(props => ({
  style: { width: `calc(${props.value || 0}% - 2px)` }
}))`
  background: #000e7a;
  position: absolute;
  top: 1px;
  bottom: 1px;
  left: 1px;
`;

function SimpleProgressBar({ value }) {
  return (
    <Container>
      <Inner value={value} />
    </Container>
  );
}

SimpleProgressBar.propTypes = {
  value: PropTypes.number
};

export default SimpleProgressBar;
