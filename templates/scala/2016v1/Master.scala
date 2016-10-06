import java.io.File
import java.nio.file.{Files, Paths, StandardCopyOption}

import scala.collection.immutable

/**
  * Created by Andrew on 2016/10/03.
  */
object Master {
  def showUsage(): Unit = {
    println("USAGE: [problem]")
    println()
    println("problem: a|b|...")
    println()
  }

  def main(args: Array[String]): Unit = {
    try {
      if (args.length >= 2) showUsage() else {
        val problem = if (args.length == 1) args(0).toUpperCase() else {
          showUsage();
          println("Choose a problem to solve interactively or press ENTER to exit:")
          println()
          io.StdIn.readLine().toUpperCase()
        }
        if (problem.isEmpty) {
          println("No problem selected.");
          println();
          println("Exiting...");
        } else {
          println()
          println(s"Read problem $problem from stdin (or type inputs below):")
          runInteractively(problem)
          println()
          println("DONE!")
        }
      }
    }
    catch {
      case e: Throwable => println(e)
    }

    def runInteractively(problem: String): Unit = getProblem(problem).processStdInOut()

    // To customize, enable relevant problem objects below...

    def getProblem(problem: String): { def processStdInOut(): Unit } = {
      problem match {
        // case "A" => ProblemA
        // case "B" => ProblemB
        // case "C" => ProblemC
        // case "D" => ProblemD
        // case "E" => ProblemE
        case _ => throw new IllegalArgumentException(s"Unsupported problem: $problem")
      }
    }
  }
}
