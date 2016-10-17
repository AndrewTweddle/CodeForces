import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemB {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  import math._

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {

    val line1 = lines.next()
    val line1Strings = line1.split(" ")
    val n = line1Strings(0).toInt
    val k = line1Strings(1).toInt
    val line2 = lines.next()
    val aValues = line2.split(" ").map(_.toInt)
    val soln = solve(n, k, aValues)
    bw.write(soln.extraWalks.toString)
    bw.newLine()
    bw.write(soln.b.mkString(" "))
    bw.newLine()
  }

  case class Solution(val extraWalks: Int, val b: Seq[Int])

  def solve(n: Int, k: Int, a: IndexedSeq[Int]): Solution = {
    case class Accumulator(walksOnPreviousDay: Int, solution: Solution)

    val b = Array.ofDim[Int](n)
    b(0) = a(0)
    for (i <- 1 until n) {
      b(i) = math.max(a(i), k - b(i - 1))
    }
    val extraWalks = a.zip(b).map(x => x._2 - x._1).sum
    Solution(extraWalks, b)
  }
}
