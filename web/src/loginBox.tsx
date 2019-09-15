import Visibility from "@material-ui/icons/Visibility";
import clsx from "clsx";
import VisibilityOff from "@material-ui/icons/VisibilityOff";
import React from "react";
import Input from "@material-ui/core/Input";
import { makeStyles } from "@material-ui/styles";
import {
  Theme,
  createStyles,
  FormControl,
  InputLabel,
  InputAdornment,
  IconButton,
  Tooltip,
  Button
} from "@material-ui/core";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: "flex",
      flexWrap: "wrap",
      flexDirection: "column"
    },
    margin: {
      margin: theme.spacing(1),
      width: 250
    },
    withoutLabel: {
      marginTop: theme.spacing(3)
    },
    textField: {
      flexBasis: 200
    }
  })
);

interface State {
  phoneNum: string;
  authCode: string;
  password: string;
  canAuth: boolean;
  showPassword: boolean;
}

interface CanAuthProps{
  canAuth: boolean;
  timeForAuth: number;
  handleClick: () => void;
}

function authTimeButton(props: CanAuthProps) {
  const [time, setTime] = React.useState<Number>(60);//can hook place in here?
  if (props.canAuth) {
    return (
      <Tooltip title="Auth" >
        <Button onClick={() => {
          //TODO
        }}>Click Here To Auth</Button>
      </Tooltip>
    );
  } else {
    return (
      <Tooltip title="Auth">
        <Button >{props.timeForAuth}s Remain</Button>
      </Tooltip>
    );
  }
}

export default function LoginBox() {
  const classes = useStyles();
  const [values, setValues] = React.useState<State>({
    phoneNum: "",
    authCode: "",
    password: "",
    showPassword: false,
    canAuth: true,
  });

  const handleChange = (prop: keyof State) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    setValues({ ...values, [prop]: event.target.value });
  };
  const handleClickShowPassword = () => {
    setValues({ ...values, showPassword: !values.showPassword });
  };

  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };

  const handleClickAuthCode = () => {
    //TODO
  }

  return (
    <div className={classes.root}>
      <FormControl className={classes.margin}>
        <InputLabel htmlFor="adornment-phone">phoneNumber</InputLabel>
        <Input
          id="adornment-phone"
          value={values.phoneNum}
          onChange={handleChange("phoneNum")}
          startAdornment={<InputAdornment position="start">+86</InputAdornment>}
        ></Input>
      </FormControl>
      <FormControl className={classes.margin}>
        <InputLabel htmlFor="adornment-authcode">authCode</InputLabel>
        <Input
          id="adornment-code"
          value={values.phoneNum}
          onChange={handleChange("authCode")}
          startAdornment={<InputAdornment position="start"></InputAdornment>}
          endAdornment={<Tooltip title="time"><Button>{}</Button></Tooltip>}
        ></Input>
      </FormControl>
      <FormControl className={clsx(classes.margin, classes.textField)}>
        <InputLabel htmlFor="adornment-password">Password</InputLabel>
        <Input
          id="adornment-password"
          type={values.showPassword ? "text" : "password"}
          value={values.password}
          onChange={handleChange("password")}
          startAdornment={<InputAdornment position="start"></InputAdornment>}
          endAdornment={
            <InputAdornment position="end">
              <IconButton
                aria-label="toggle password visibility"
                onClick={handleClickShowPassword}
                onMouseDown={handleMouseDownPassword}
              >
                {values.showPassword ? <Visibility /> : <VisibilityOff />}
              </IconButton>
            </InputAdornment>
          }
        />
      </FormControl>
    </div>
  );
}
