import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemA {
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
  @tailrec
  def gcd(a: Int, b: Int): Int = if (b == 0) a else gcd(b, a % b)
  def lcm(a: Int, b: Int): Int = a * b / gcd(a, b)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {

    val line = lines.next()
    val strings = line.split(" ")
    val soln = solve(strings(0).toInt, strings(1).toInt)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(k: Int, r: Int): Int = {
    def solveWithRCoin(): Option[Int] = {
      val d = k % 10
      (1 to 9).collectFirst{
        case n if (n * d) % 10 == r => n
      }
    }

    val minWithoutRCoin = lcm(10, k) / k
    val minWithRCoin = solveWithRCoin()
    minWithRCoin match {
      case Some(n) => math.min(n, minWithoutRCoin)
      case _ => minWithoutRCoin
    }
  }
}
