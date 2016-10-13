import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemC {
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

  case class Signal(val x: Int, val y: Int)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val strings = line.split(' ')
    val n = strings(0).toInt
    val m = strings(1).toInt
    val k = strings(2).toInt
    val signals = Array.ofDim[Signal](k)
    for (i <- 0 until k) {
      val row = lines.next()
      val coords = row.split(' ').map(_.toInt)
      signals(i) = Signal(coords(0), coords(1))
    }

    val firstTimeEachSignalIsHit = solve(n, m, k, signals)
    firstTimeEachSignalIsHit.map {
      case Some(tme) => tme.toString
      case _ => "-1"
    }.foreach { str =>
      bw.write(str)
      bw.newLine()
    }
  }

  @tailrec
  def gcd(a: Long, b: Long): Long = if (b == 0) a else gcd(b, a % b)
  def lcm(a: Long, b: Long): Long = a * b / gcd(a, b)

  def getShortestTime(n: Int, m: Int, sig: Signal, maxTime: Long): Option[Long] = {
    // Consider the four reflections of the point(x, y) about the centre line of a 2n by 2m box
    // i.e. n +/- (n - x), m +/- (m - y)
    @tailrec
    def timeToReachSignalFromBlock(xBlock: Long, yOffsetFromPreviousBlock: Int): Option[Long] = {
      def isSignalReached(t: Long, yOffset: Int): Boolean =
        if (t >= maxTime) false else {
          // val yOffsetFromBlock = yOffsetFromPreviousBlock % (2 * m)
          (yOffset == sig.y) || (yOffset == 2 * m - sig.y)
        }

      if (xBlock >= maxTime) None else {
        val t1 = xBlock + sig.x
        val y1 = (yOffsetFromPreviousBlock + sig.x) % (2 * m)
        val t2 = xBlock + 2 * n - sig.x
        val y2 = (yOffsetFromPreviousBlock + 2 * n - sig.x) % (2 * m)
        if (isSignalReached(t1, y1)) Some(t1) else if (isSignalReached(t2, y2)) Some(t2) else {
          val newYOffsetFromPreviousBlock = (yOffsetFromPreviousBlock + 2 * n) % (2 * m)
          timeToReachSignalFromBlock(xBlock + 2 * n, newYOffsetFromPreviousBlock)
        }
      }
    }

    // x and y must be congruent modulo 2 (both even or both odd):
    if ((sig.x % 2) != (sig.y % 2)) None else timeToReachSignalFromBlock(0, 0)
  }

  def solve(n: Int, m: Int, k: Int, signals: Array[Signal]): Array[Option[Long]] = {
    if (n > m) {
      // Switch axes if n > m, so that we don't accidentally skip blocks:
      def swapAxis(sig: Signal) = sig.copy(x = sig.y, y = sig.x)
      solve(m, n, k, signals.map(swapAxis))
    } else {
      // Iterate through blocks of size n by n, where n < m:
      val rayTime = lcm(n, m)
      val shortestTimes = signals.map(getShortestTime(n, m, _, rayTime))
      shortestTimes
    }
  }
}
