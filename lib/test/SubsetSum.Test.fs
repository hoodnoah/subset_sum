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

    let genOkInput: Gen<Input> =
        gen {
            let! elemList = gen10ElementList
            let! itemsToSum = Gen.subListOf elemList // randomly select elements from floatList

            let target =
                itemsToSum
                |> List.sumBy (fun { Value = value } -> value)

            let elemSeq = Seq.ofList elemList

            return { Elements = elemSeq; Target = target }
        }

    let genBadInput: Gen<Input> =
        gen {
            let! elemList = gen10ElementList

            let target =
                elemList
                |> List.sumBy (fun { Value = value } -> value)
                |> (fun x -> x + 1.0) // add one to sum of all values

            let elemSeq = Seq.ofList elemList

            return { Elements = elemSeq; Target = target }
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

let sanityCheck =
    testCase "returns a sane result given a specific input"
    <| fun _ ->
        let inputElems =
            [ { Label = "01"; Value = 1.0 }
              { Label = "02"; Value = 2.0 }
              { Label = "03"; Value = 3.0 } ]
            |> Seq.ofList

        let target = 5.0

        let expectedResultElements: seq<Element> =
            [ { Label = "02"; Value = 2.0 }
              { Label = "03"; Value = 3.0 } ]
            |> Seq.ofList

        let expectedResult =
            { Output.Elements = expectedResultElements
              Output.Target = target }

        let input =
            { Input.Elements = inputElems
              Target = target }

        let result = subsetSum input

        match result with
        | Ok r -> compareOutput r expectedResult
        | _ -> failtest "Did not return an Ok result when one was expected."

let sanityCheck2 =
    testCase "returns a sane result given a specific input containing a negative number"
    <| fun _ ->
        let inputElems =
            [ { Label = "01"; Value = -1.0 }
              { Label = "02"; Value = 2.0 }
              { Label = "03"; Value = 3.0 } ]
            |> Seq.ofList

        let target = -1.0

        let expectedResultElements: seq<Element> =
            [ { Label = "01"; Value = -1.0 } ] |> Seq.ofList

        let expectedResult =
            { Output.Elements = expectedResultElements
              Output.Target = target }

        let input =
            { Input.Elements = inputElems
              Target = target }

        let result = subsetSum input

        match result with
        | Ok r -> compareOutput r expectedResult
        | _ -> failtest "Did not return an Ok result when one was expected."


[<Tests>]
let AllTests =
    testList
        "allTests"
        [

          resultElementsInOriginalList
          resultSumsToTarget
          sanityCheck
          sanityCheck2

          ]
