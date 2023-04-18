import { FC } from "react";
import { Link } from "react-router-dom";

export default (): ReturnType<FC> => {
  return (
    <div>
      <h1>404 Page</h1>
      <Link to="/">Go Home</Link>
    </div>
  );
};
