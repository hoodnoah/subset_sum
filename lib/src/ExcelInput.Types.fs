module Domain.ExcelInput

open Domain.SubsetSum

type ExcelInput =
    { InputElements: Element seq
      Targets: float seq }

type InputFilepath = InputFilepath of string

type ElementLabelReadError = ElementLabelReadError of string
type ElementValueReadError = ElementValueReadError of string

type ElementReadError =
    | ElementLabelReadError
    | ElementValueReadError

type TargetReadError = TargetReadError of string

type ExcelInputError =
    | ElementReadError
    | TargetReadError

type StreamReader = InputFilepath -> System.IO.FileStream

type ReadInput = StreamReader -> InputFilepath -> Result<ExcelInput, ExcelInputError>
