import React, { useState, useRef, useLayoutEffect } from "react";
import styled from "styled-components";
import PropTypes from "prop-types";
import { Modal, Icon, Button } from "@react95/core";

import FakeProgressBar from "../FakeProgressBar/FakeProgressBar";

const LoaderModal = styled(Modal)`
  z-index: 10000;
  top: ${props => props.top}px;
  left: ${props => props.left}px;
`;

const Content = styled.div`
  display: flex;
  padding: 8px;

  > :first-child {
    flex-grow: 0;
    padding-right: 16px;
  }

  > :last-child {
    flex-grow: 1;

    p {
      margin-bottom: 8px;
    }
  }
`;

const Actions = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

function Loader({
  modalIcon,
  modalTitle,
  modalWidth,
  contentIcon,
  contentText,
  errorIcon,
  errorText,
  loading,
  error,
  onRetry
}) {
  const [position, setPosition] = useState({ x: 0, y: 0 });

  const layoutElem = useRef(null);
  useLayoutEffect(() => {
    if (!layoutElem.current) {
      return;
    }

    const modalElem = layoutElem.current.parentElement.parentElement;

    setPosition({
      x: Math.floor((window.innerWidth - modalElem.clientWidth) / 2),
      y: Math.floor((window.innerHeight - modalElem.clientHeight) / 2)
    });
  }, [loading, error, contentText, errorText]);

  if (!loading && !error) {
    return null;
  }

  return (
    <LoaderModal
      icon={modalIcon}
      title={modalTitle}
      width={modalWidth}
      height="auto"
      defaultPosition={{ x: 0, y: 0 }}
      top={position.y}
      left={position.x}
    >
      <Content ref={layoutElem}>
        <div>
          <Icon name={error ? errorIcon : contentIcon} />
        </div>
        {error && (
          <div>
            <p>{errorText}</p>
          </div>
        )}
        {!error && (
          <div>
            <p>{contentText}</p>
            <FakeProgressBar />
          </div>
        )}
      </Content>
      {error && onRetry && (
        <Actions>
          <Button onClick={onRetry}>Reload</Button>
        </Actions>
      )}
    </LoaderModal>
  );
}

Loader.propTypes = {
  modalIcon: PropTypes.string,
  modalTitle: PropTypes.string,
  modalWidth: PropTypes.number,
  contentIcon: PropTypes.string,
  contentText: PropTypes.node,
  errorIcon: PropTypes.string,
  errorText: PropTypes.node,
  loading: PropTypes.bool,
  error: PropTypes.any,
  onRetry: PropTypes.func
};

Loader.defaultProps = {
  modalIcon: "bat_wait",
  modalTitle: "Loading",
  modalWidth: 348,
  contentIcon: "explore",
  contentText:
    "Please wait while we load the data for you. It may take a few seconds, please be patient.",
  errorIcon: "warning",
  errorText: "Something went wrong, please try again in a few moments."
};

export default Loader;
