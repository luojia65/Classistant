import * as React from "react";
import { RouteComponentProps, Route, Switch } from "react-router-dom";
import LoginIndexPage from "./login/index";
import { connect } from "react-redux";



// Combine both state + dispatch props - as well as any props we want to pass - in a union type.
type AllProps = RouteComponentProps;

const LoginPage: React.FC<AllProps> = ({ match }) => {
  console.log(match);
  return (
    <Switch>
      <Route exact path={`${match.path}/`} component={LoginIndexPage} />
    </Switch>
  );
};

// It's usually good practice to only include one context at a time in a connected component.
// Although if necessary, you can always include multiple contexts. Just make sure to
// separate them from each other to prevent prop conflicts.


// Now let's connect our component!
// With redux v4's improved typings, we can finally omit generics here.
export default connect()(LoginPage);