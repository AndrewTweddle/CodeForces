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

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val parms = line.split(" ").map(_.toInt)
    val n = parms(0)
    val n1 = parms(1)
    val n2 = parms(2)
    val as = lines.next().split(" ").map(_.toLong)
    val soln = solve(n, n1, n2, as)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(n: Int, n1: Int, n2: Int, as: Array[Long]): Double = {
    val nMin = math.min(n1, n2)
    val nMax = math.max(n1, n2)
    val asorted = as.map(- _).sorted
    val (aMin, aRest) = asorted.splitAt(nMin)
    val aMax = aRest.take(nMax)
    val mean1 = - aMin.sum / nMin.toDouble
    val mean2 = - aMax.sum / nMax.toDouble
    val sumOfMeans = mean1 + mean2
    sumOfMeans
  }
}
