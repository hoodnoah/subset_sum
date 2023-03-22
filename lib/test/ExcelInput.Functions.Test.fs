module Tests.ExcelInputFunctionsTest

// System libraries
open System.IO

// testing libraries
open Expecto
open FsCheck

// module(s) under test
open Domain.ExcelInput
open Domain.SubsetSum
open Functions.ExcelInput

[<Literal>]
let testFilePath = "test_fixtures/testInput.xlsx"

let readOnlyReader (InputFilepath filepath) =
    File.Open(filepath, FileMode.Open, FileAccess.Read)

let readsExampleFileCorrectly =
    testCase "Reads an example excel file as expected"
    <| fun _ ->
        let expectedElements =
            [ ("Element 1", 1)
              ("Element 2", 2)
              ("Element 3", 3)
              ("Element 4", 4)
              ("Element 5", 6)
              ("Element 6", 5)
              ("Element 7", 7)
              ("Element 8", 8)
              ("Element 9", 9)
              ("Element 10", 10) ]
            |> Seq.ofList
            |> Seq.map (fun (l, v) -> { Element.Label = l; Value = float v })

        let expectedTargets =
            [ 1; 3; 5; 7; 9 ] |> Seq.ofList |> Seq.map float

        let expectedResult =
            { InputElements = expectedElements
              Targets = expectedTargets }

        let result =
            readInput readOnlyReader (InputFilepath testFilePath)

        Expect.equal result (Ok expectedResult) "Did not return the expected result"


[<Tests>]
let tests =
    testList
        "ExcelInputFunctionsTests"
        [

          readsExampleFileCorrectly

         ]
