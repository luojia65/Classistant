import React from "react";
import "./App.css";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import SignIn from "./pages/Signin";
import SignUp from "./pages/Signup";
import DashBoard from "./pages/DashBoard";

const App: React.FC = () => {
  return (
    <div className="App">
      <Router>
        <Switch>
          <Route exact path="/signin" component={SignIn} />
          <Route exact path="/signup" component={SignUp} />
          <Route exact path="/" component={DashBoard} />
        </Switch>
      </Router>
    </div>
  );
};

export default App;
