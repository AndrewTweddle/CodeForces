import scala.io._
import java.io._

object ProblemA {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = io.Source.fromInputStream(System.in)
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
        bw.close()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  case class Solution(runCount: Int, runs: List[Int])

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {

    val size = lines.next().toInt
    val chars = lines.next() + 'W' // Ensure last run isn't missed by appending dummy character
    val soln = solve(size, chars.toList)
    bw.write(s"${soln.runCount}")
    bw.newLine()
    if (soln.runCount > 0) {
      bw.write(soln.runs.mkString(" "))
      bw.newLine()
    }
  }

  case class Accumulator(wasLastCharB: Boolean, blackCount: Int, runsInReverse: List[Int], runCount: Int)

  def solve(size: Int, chars: List[Char]): Solution = {
    val accumulator = chars.foldLeft(Accumulator(false, 0, List.empty, 0)) { (acc: Accumulator, ch: Char) =>
        if (ch == 'B') {
          acc.copy(wasLastCharB = true, blackCount = acc.blackCount + 1)
        } else {
          if (acc.wasLastCharB) {
            acc.copy(wasLastCharB = false, blackCount = 0,
              runsInReverse = acc.blackCount :: acc.runsInReverse,
              runCount = acc.runCount + 1)
          } else {
            acc
          }
        }
    }
    Solution(accumulator.runCount, accumulator.runsInReverse.reverse)
  }
}
