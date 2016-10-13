import java.io._

import scala.io.Source
import scala.annotation.tailrec
import scala.collection.mutable.HashSet

object ProblemC3 {
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

  case class Point(val x: Int, val y: Int)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val strings = line.split(' ')
    val n = strings(0).toInt
    val m = strings(1).toInt
    val k = strings(2).toInt
    val signals = Array.ofDim[Point](k)
    for (i <- 0 until k) {
      val row = lines.next()
      val coords = row.split(' ').map(_.toInt)
      signals(i) = Point(coords(0), coords(1))
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

  def solve(n: Int, m: Int, k: Int, signals: Array[Point]): Array[Option[Long]] = {
    if (n > m) {
      // Switch axes if n > m, so that we don't accidentally skip blocks:
      def swapAxis(sig: Point) = sig.copy(x = sig.y, y = sig.x)
      solve(m, n, k, signals.map(swapAxis))
    } else {
      // Iterate through blocks of size n by n, where n < m:
      val maxTime = lcm(n, m)
      val indexedSignalsToFind = signals.zipWithIndex.filter { sigWithIndex =>
        val (point, _) = sigWithIndex
        (point.x % 2) == (point.y % 2)
      }
      val remainingSignalIndices: HashSet[Int] = HashSet(indexedSignalsToFind.map(_._2) : _*)
      val signalTimes = Array.fill[Option[Long]](k)(None)

      @tailrec
      def findAllSignals(xBlock: Long): Unit = {
        def isSignalReached(index: Int, sig: Point, t: Long): Boolean =
          if (t >= maxTime) false else {
            val yOffsetFromBlock = t % (2 * m)
            if ((yOffsetFromBlock == sig.y) || (yOffsetFromBlock == 2 * m - sig.y)) {
              remainingSignalIndices.remove(index)
              signalTimes(index) = Some(t)
              true
            } else false
          }

        if ((xBlock < maxTime) && !remainingSignalIndices.isEmpty) {
          val remSignals = remainingSignalIndices.toList  // copy, as elements will be removed if found
          remSignals.foreach { (index: Int) =>
            val sig = signals(index)
            val t1 = xBlock + sig.x
            if (!isSignalReached(index, sig, t1)) {
              val t2 = xBlock + 2 * n - sig.x
              val dummy = isSignalReached(index, sig, t2)
            }
          }
          // recursively look for signals in the next n by n block:
          findAllSignals(xBlock + 2 * n)
        }
      }

      findAllSignals(0)
      signalTimes
    }
  }
}
