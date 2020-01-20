import React, { useEffect, useState, useRef } from "react";

import SimpleProgressBar from "../SimpleProgressBar/SimpleProgressBar";

function useInterval(callback, delay) {
  const savedCallback = useRef();

  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  useEffect(() => {
    if (delay !== null) {
      const id = setInterval(() => {
        savedCallback.current(() => clearInterval(id));
      }, delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}

function FakeSimpleProgressBar(props) {
  const [percent, setPercent] = useState(0);

  useInterval(cancel => {
    const newPercent = percent + Math.floor(Math.random() * 4);

    if (newPercent >= 100) {
      setPercent(100);
      cancel();
      return;
    }

    setPercent(newPercent);
  }, 100);

  return <SimpleProgressBar {...props} value={percent} />;
}

export default FakeSimpleProgressBar;
