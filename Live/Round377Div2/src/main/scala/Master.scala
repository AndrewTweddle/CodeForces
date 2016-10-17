import scala.io.Source
import org.scalameter._

/**
  * Created by Andrew on 2016/10/03.
  */
object Master {
  val TIMER_OPTION = "--timer"

  def showUsage(): Unit = {
    println(s"USAGE: [problem] [$TIMER_OPTION]")
    println()
    println("problem: a|b|...")
    println()
  }

  def main(args: Array[String]): Unit = {
    val isTimed = (args.length == 2) && (args(1) == TIMER_OPTION)
    if ((args.length >= 2) && !isTimed) showUsage()
    else {
      val problem = if (args.length >= 1) args(0).toUpperCase()
      else {
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
        run(problem, isTimed)
        println()
        println("DONE!")
      }
    }
  }

  def run(problem: String, isTimed: Boolean): Unit = {
    val solver = getProblem(problem)
    if (isTimed) {
      val src = io.Source.fromInputStream(System.in)
      try {
        // Timing should exclude the time required to read from the standard input stream.
        // So read the lines into a string, then process the string as a stream:
        val lines = src.getLines().takeWhile(!_.isEmpty).mkString("", System.lineSeparator(), System.lineSeparator())
        val stringSrc = Source.fromString(lines)
        val time = measure { solver.processFromSource(stringSrc) }
        println()
        println(s"Duration: $time")
      } finally {
        src.close();
      }
    } else {
      solver.processStdInOut()
    }
  }

  // To customize, enable relevant problem objects below...

  def getProblem(problem: String): {
    def processStdInOut(): Unit
    def processFromSource(source: Source): Unit
  } = {
    problem match {
      case "A" => ProblemA
      case "B" => ProblemB
      case "C" => ProblemC
      case _ => throw new IllegalArgumentException(s"Unsupported problem: $problem")
    }
  }
}
