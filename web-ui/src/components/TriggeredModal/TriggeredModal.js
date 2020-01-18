import React, { useState, useCallback, Fragment } from "react";
import PropTypes from "prop-types";

function TriggeredModal({ trigger, modal }) {
  const [open, setOpen] = useState(false);

  const handleTriggerClick = useCallback(() => {
    setOpen(true);
  }, [setOpen]);

  const handleModalClose = useCallback(() => {
    setOpen(false);
  }, [setOpen]);

  return (
    <Fragment>
      {trigger(handleTriggerClick)}
      {open && modal(handleModalClose)}
    </Fragment>
  );
}

TriggeredModal.propTypes = {
  trigger: PropTypes.func.isRequired,
  modal: PropTypes.func.isRequired
};

export default TriggeredModal;
