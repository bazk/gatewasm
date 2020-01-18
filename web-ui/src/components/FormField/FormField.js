import styled from "styled-components";

const FormField = styled.div`
  display: flex;
  align-items: center;
  margin: 4px 0;
`;

FormField.Label = styled.label`
  flex: 1;
`;

FormField.Input = styled.div`
  flex: 3;

  display: flex;
  > :first-child {
    flex-grow: 1;
  }
`;

export default FormField;
