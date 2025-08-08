import React from "react";

export default function Page() {
  return (
    <div>
      React
      <Test />
    </div>
  );
}

function Test() {
  return React.createElement("div", null, "React 2");
}
