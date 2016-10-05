import java.io._
import collection.immutable._
import scala.collection.mutable.ArrayBuffer

object ProblemC {
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

  case class Solution(bestMinBj: Int, minChangeCount: Int, newPlayList: Array[Int])

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val params = lines.next().split(' ').map(_.toInt)
    val n = params(0)
    val m = params(1)
    val performers = lines.next().split(' ').map(_.toInt)
    val solution = solve(n, m, performers)
    bw.write(s"${solution.bestMinBj} ${solution.minChangeCount}")
    bw.newLine()
    bw.write(solution.newPlayList.mkString(" "))
    bw.newLine()
  }

  def solve(n: Int, m: Int, performers: Array[Int]): Solution = {
    val target = n / m  // The maximum value for the minimum number of performances by a preferred band

    // The queues store the zero-based indexes of performances which can be replaced without decreasing the target:
    var queueA = ArrayBuffer[Int]()  // Indexes for non-preferred bands
    val queueB = ArrayBuffer[Int]()  // Indexes for preferred bands that have exceeded the target

    // Count performances by preferred bands:
    val bandCounts = Array.ofDim[Int](m)
    for (i <- 1 to n) {
      val j = performers(i - 1)
      if (j > m) {
        queueA += (i - 1)
      } else {
        val newBandCount = bandCounts(j - 1) + 1
        bandCounts(j - 1) = newBandCount
        if (newBandCount > target) {
          queueB += (i - 1)
        }
      }
    }

    val deltas = bandCounts.map(bc => if (bc >= target) 0 else target - bc)
    val minChangeCount = deltas.sum
    val replacementValues = deltas.zipWithIndex.flatMap {
      deltaAndIndex => IndexedSeq.fill(deltaAndIndex._1)(deltaAndIndex._2 + 1)
    }

    queueA ++= queueB  // Merge the queues, but replace non-preferred bands first
    val replacements = queueA.take(minChangeCount).zip(replacementValues)
    replacements.foreach{ indexAndValue:(Int, Int) => performers(indexAndValue._1) = indexAndValue._2 }

    Solution(target, minChangeCount, performers)
  }
}
