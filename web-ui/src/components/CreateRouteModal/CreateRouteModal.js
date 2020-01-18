import React from "react";
import styled from "styled-components";
import { Modal, List, Fieldset, Input, Alert } from "@react95/core";
import { useForm, Controller } from "react-hook-form";

import FormField from "../FormField/FormField";
import FileInput from "../FileInput/FileInput";
import Dropdown from "../Dropdown/Dropdown";

const Form = styled.form`
  padding: 0 8px 8px 8px;
`;

function CreateRouteModal({ onSubmit, onClose, ...props }) {
  const { register, handleSubmit, errors, clearError, control } = useForm({
    reValidateMode: "onSubmit"
  });

  return (
    <Modal
      icon="bat_exec"
      title="New Route"
      width={360}
      height={224}
      closeModal={onClose}
      buttons={[
        {
          value: "Ok",
          onClick: handleSubmit(onSubmit)
        },
        { value: "Cancel", onClick: onClose }
      ]}
      menu={[
        {
          name: "File",
          list: (
            <List>
              <List.Item onClick={onClose}>Exit</List.Item>
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
      {...props}
    >
      <Fieldset legend="Route">
        <Form onSubmit={handleSubmit(onSubmit)}>
          {(errors.path || errors.method || errors.handler) && (
            <Alert
              title="Error"
              message={
                (errors.path && errors.path.message) ||
                (errors.method && errors.method.message) ||
                (errors.handler && errors.handler.message)
              }
              type="error"
              onCloseModal={() => clearError()}
              closeAlert={() => clearError()}
            />
          )}

          <FormField>
            <FormField.Label>Path:</FormField.Label>
            <FormField.Input>
              <Input
                type="text"
                name="path"
                ref={register({
                  required: "Please choose a valid path",
                  pattern: {
                    value: /^\/[^\s]*$/,
                    message: 'Please input a valid path, e.g. "/some/api".'
                  }
                })}
                defaultValue="/"
              />
            </FormField.Input>
          </FormField>
          <FormField>
            <FormField.Label>Method:</FormField.Label>
            <FormField.Input>
              <Controller
                as={
                  <Dropdown
                    options={["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"]}
                  />
                }
                name="method"
                control={control}
                defaultValue="GET"
                rules={{ required: "Please select a valid HTTP method" }}
              />
            </FormField.Input>
          </FormField>
          <FormField>
            <FormField.Label>Handler:</FormField.Label>
            <FormField.Input>
              <Controller
                as={<FileInput />}
                name="handler"
                control={control}
                rules={{
                  required:
                    "Please select a valid .wasm binary file as a handler"
                }}
              />
            </FormField.Input>
          </FormField>
        </Form>
      </Fieldset>
    </Modal>
  );
}

export default CreateRouteModal;
