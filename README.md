# Flow-Model

Simple application to estimate a free flow (on non-compressive fluid) from a repository. In essence, the flow 
estimation are solving using the Bernoulli Equation. 

## How To Use
For use **Flow-Model** its necessary create three json file. In the configuration file, in 
this file you must set a solver mode, which on is designed to solve one of the Bernoulli's
terms, currently there are three of them:
- `Velocity`;
- `Pressure`;
- `Height`.


Firstly, the configuration file, which must follow the template: 

```json
{
  "save_result": true,
  "show_result": true,
  "solver_mode": "Velocity",
  "inlet_file_path": "/path/to/your/inlet.json",
  "outlet_file_path": "/path/to/your/outlet.json",
  "streamline_file_path": "/path/to/your/system_definition.json",
  "result_file_path": "/path/to/your/result.json"
}
```

Secondly its necessary to define a surface file (one for inlet and one for outlet). 
This file must contain the following structure: 
```json
{
  "pressure": 0,
  "velocity": 0,
  "height": 0
}
```

And finally, there is a system properties configuration file, with the following fields:
```json
{
  "density": 1000,
  "fluid_column_height": 0,
  "gravity_acceleration": 9.81
}
```

To run **Flow-Model** you just need to pass the configuration file path:
```shell
./Flow-Model /path/to/configuration.json
```