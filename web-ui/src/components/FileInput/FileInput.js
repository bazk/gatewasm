import React, { useState, useCallback } from "react";
import PropTypes from "prop-types";
import styled from "styled-components";
import { Input, Button } from "@react95/core";
import { useDropzone } from "react-dropzone";

const Wrapper = styled.div`
  display: flex;
`;

const HiddenInput = styled.input`
  display: none;
`;

const VisibleInput = styled(Input)`
  flex-shrink: 1;
  flex-grow: 1;
  width: 0;
  margin-right: 8px;
`;

const BrowseButton = styled(Button)``;

function FileInput({ value, onChange, buttonText }) {
  const [innerVal, setInnerVal] = useState(null);

  const onDrop = useCallback(
    acceptedFiles => {
      if (acceptedFiles.length === 0) {
        return;
      }

      setInnerVal(acceptedFiles[0]);
      onChange && onChange(acceptedFiles[0]);
    },
    [onChange]
  );

  const { getRootProps, getInputProps } = useDropzone({ onDrop });

  const innerFileName = innerVal ? innerVal.name : "";
  const fileName = value ? value.name : innerFileName;

  return (
    <Wrapper {...getRootProps()}>
      <HiddenInput {...getInputProps()} type="file" />
      <VisibleInput type="text" readOnly value={fileName} />
      <BrowseButton type="button">{buttonText}</BrowseButton>
    </Wrapper>
  );
}

FileInput.propTypes = {
  value: PropTypes.instanceOf(File),
  onChange: PropTypes.func,
  buttonText: PropTypes.string
};

FileInput.defaultProps = {
  buttonText: "Browse..."
};

export default FileInput;
