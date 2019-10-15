import * as React from 'react'
import { connect } from 'react-redux'
import { ThemeProvider } from 'emotion-theming'

import { ApplicationState } from '../store'
import { ThemeColors } from '../store/layout'
import * as themes from '../styles/theme'

interface PropsFromState {
  theme: ThemeColors
}

type AllProps = PropsFromState

const MyThemeProvider: React.FC<AllProps> = props => {
  const { theme, children } = props
  return <ThemeProvider theme={themes[theme]}>{children}</ThemeProvider>
}

const mapStateToProps = ({ layout }: ApplicationState) => ({
  theme: layout.theme
})

export default connect(mapStateToProps)(MyThemeProvider)
