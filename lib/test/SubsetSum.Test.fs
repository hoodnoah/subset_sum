module SubsetSum.Test

// testing libraries
open Expecto
open FsCheck

let FloatCompAccuracy = Accuracy.low

// module(s) under test
open Domain.SubsetSum
open Functions.SubsetSum

type OkInput = OkInput of Input
type ErrorInput = ErrorInput of Input


module InputGen =
    let genNNFloat: Gen<float> =
        gen {
            let! nnFlt = Arb.generate<NormalFloat>

            return nnFlt.Get
        }

    let genNonNullString: Gen<string> =
        gen {
            let! nnStr = Arb.generate<NonNull<string>>

            return nnStr.Get
        }

    let genItem: Gen<Element> =
        gen {
            let! label = genNonNullString
            let! value = genNNFloat

            return { Label = label; Value = value }
        }

    let gen10ElementList: Gen<Element list> =
        gen {
            return!
                Gen.sized (fun size ->
                    let maxSize = min (max 1 size) 10
                    Gen.listOfLength maxSize genItem)
        }

    let genOkInput: Gen<OkInput> =
        gen {
            let! elemList = gen10ElementList
            let! itemsToSum = Gen.subListOf elemList // randomly select elements from floatList

            let target =
                itemsToSum
                |> List.sumBy (fun { Value = value } -> value)

            let elemSeq = Seq.ofList elemList

            return
                { Input.Elements = elemSeq
                  Input.Target = target }
                |> OkInput
        }

    let genBadInput: Gen<ErrorInput> =
        gen {
            let! elemList = gen10ElementList

            let target =
                elemList
                |> List.map (fun item -> { item with Value = abs item.Value })
                |> List.sumBy (fun { Value = value } -> value)
                |> (fun x -> x + 1.0)

            let elemSeq = Seq.ofList elemList

            return
                { Input.Elements = elemSeq
                  Input.Target = target }
                |> ErrorInput
        }

    type InputGen() =
        static member Element() = Arb.fromGen genItem
        static member OkInput() = Arb.fromGen genOkInput
        static member ErrorInput() = Arb.fromGen genBadInput

let compareOutput (output1: Output) (output2: Output) =
    Expect.equal output1.Target output2.Target "Output targets didn't match"
    Expect.sequenceEqual output1.Elements output2.Elements "Elements were not equal,"

let elementExistsInSequence sequence element = Seq.contains element sequence

let config =
    { FsCheckConfig.defaultConfig with
        maxTest = 1000
        arbitrary = [ typeof<InputGen.InputGen> ] }

let resultElementsInOriginalList =
    testPropertyWithConfig config "Elements returned by SubsetSum exist in the input space"
    <| fun (OkInput input) ->
        let result = subsetSum input

        match result with
        | Ok r ->
            let allResultInInput =
                r.Elements
                |> Seq.forall (fun x -> Seq.contains x input.Elements)

            Expect.isTrue allResultInInput "returned elements were not all contained in the input list."
        | _ -> failtest "Did not return an Ok result with an input which should produce one."

let resultSumsToTarget =
    testPropertyWithConfig config "Elements returned in the result sum to the target value"
    <| fun (OkInput input) ->
        let result = subsetSum input

        match result with
        | Ok r ->
            let resultSum = Seq.sumBy (fun x -> x.Value) r.Elements

            Expect.floatClose FloatCompAccuracy resultSum input.Target "Did not sum to the input's target"
        | _ -> failtest "Did not return an OK result when one was expected."

let resultErrorsWithoutSolution =
    testPropertyWithConfig config "Returns an Error result when there is no combination which sums to the Target"
    <| fun (ErrorInput input) ->
        let result = subsetSum input

        let expectedError =
            "No solution." |> NoSolutionError |> NoSolution

        match result with
        | Ok _ -> failtest "returned Ok for an input without a solution"
        | Error e -> Expect.equal e expectedError "Did not return the expected type of error"

[<Tests>]
let AllTests =
    testList
        "allTests"
        [ resultElementsInOriginalList
          resultSumsToTarget
          resultErrorsWithoutSolution ]
