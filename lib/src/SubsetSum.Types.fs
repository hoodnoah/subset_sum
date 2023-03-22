module Domain.SubsetSum

type Value = Value of float
type Element = { Label: string; Value: float }

type Input =
    { Elements: Element seq
      Target: float }

type Output =
    { Elements: Element seq
      Target: float }

type NoSolutionError = NoSolutionError of string

type SubsetSumError = NoSolution of NoSolutionError

type OutputResult = Result<Output, SubsetSumError>

type SubsetSum = Input -> OutputResult
