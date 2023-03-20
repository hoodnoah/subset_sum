module Tests.ExcelInputFunctionsTest

// testing libraries
open Expecto
open FsCheck

// module(s) under test
open Domain.ExcelInput
open Functions.ExcelInput

[<Tests>]
let tests =
    testList
        "ExcelInputFunctionsTests"
        [ testCase "passes"
          <| fun _ -> Expect.isTrue true "true is false?" ]
