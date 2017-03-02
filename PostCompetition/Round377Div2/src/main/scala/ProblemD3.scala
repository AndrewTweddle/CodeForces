import java.io._

import scala.io.Source
import scala.annotation.tailrec
import scala.collection.mutable.ListBuffer

object ProblemD3 {
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

  // Declare type aliases for readability (but not type-safety)...
  type SubjectId = Int;
  type PrepTime = Int;
  type DayIndex = Int;

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line1 = lines.next()
    val strings = line1.split(" ")
    val n = strings(0).toInt
    val m = strings(1).toInt

    val line2 = lines.next()
    val d = line2.split(" ")
      .map(_.toInt)
      .map(di => if (di == 0) None else Some(di - 1))  // zero-base the d's

    val line3 = lines.next()
    val a = line3.split(" ").map(pt => pt.toInt)

    val soln = solve(n, m, d, a)
    soln match {
      case Some(days) => bw.write((days + 1).toString)  // Make d's one-based again
      case _ => bw.write("-1")
    }
    bw.newLine()
  }

  def solve(numDays: Int, numSubjects: Int, examByDay: Array[Option[SubjectId]], prepTimes: Array[PrepTime])
    : Option[DayIndex] = {

    // Get a sorted array of exam days for each subject:
    val maxPrepTimeOfAnyExam = prepTimes.max
    val examDaysBySubject = Array.fill(numSubjects)(ListBuffer[DayIndex]())
    for (dayIndex <- 0 until numDays) {
      val subjectOpt = examByDay(dayIndex)
      if (subjectOpt.isDefined) {
        val subjectId = subjectOpt.get
        if (dayIndex >= maxPrepTimeOfAnyExam || prepTimes(subjectId) <= dayIndex) {
          examDaysBySubject(subjectId).append(dayIndex)
        }
      }
    }

    def findDayIndexOfLastExamForSubjectOnOrBefore(subjectId: SubjectId, dayIndex: DayIndex): Option[Int] = {
      val examDays = examDaysBySubject(subjectId)

      @tailrec
      def binarySearch(lowerIx: Int, upperIx: Int): Int = {
        if (lowerIx == upperIx) lowerIx else {
          val midIx = (lowerIx + upperIx + 1) / 2
          val midVal = examDays(midIx)
          if ((midIx == upperIx) || (midVal == dayIndex))
            midVal
          else if (midVal > dayIndex) binarySearch(lowerIx, midIx)
          else binarySearch(midIx, upperIx)
        }
      }

      if (examDays.isEmpty) None else {
        val lowerVal = examDays(0)
        if (lowerVal > dayIndex) None else {
          val upperIndex = examDays.length - 1
          val upperVal = examDays(upperIndex)
          if (upperVal <= dayIndex) Some(upperVal)
          else Some(binarySearch(0, upperIndex))
        }
      }
    }

    def isSolution(dayIndex: DayIndex): Boolean = {
      val subjectExamDays = (0 until numSubjects).map { subjectId =>
        subjectId -> findDayIndexOfLastExamForSubjectOnOrBefore(subjectId, dayIndex)
      }
      if (!subjectExamDays.forall(_._2.isDefined)) false else {
        val sortedSubjectExamDays = subjectExamDays.map(pair => (pair._1, pair._2.get)).sortBy(_._2)

        def loop(sortIndex: Int, prevDayIndex: Int): Boolean = {
          val sed = sortedSubjectExamDays(sortIndex)
          val nextDayIndex = prevDayIndex + prepTimes(sed._1) + 1
          if (nextDayIndex > sed._2) false else {
            val nextSortIndex = sortIndex + 1
            if (nextSortIndex == numSubjects) true else loop(nextSortIndex, nextDayIndex)
          }
        }

        loop(0, -1)
      }
    }

    // Do a binary search to find the best solution
    @tailrec
    def search(lowerBound: DayIndex, upperBound: DayIndex): DayIndex = {
      if (lowerBound == upperBound) lowerBound else {
        val middleDay: DayIndex = (lowerBound + upperBound + 1) / 2
        if (isSolution(middleDay)) search(lowerBound, middleDay)
        else if (middleDay == upperBound) upperBound
        else search(middleDay, upperBound)
      }
    }

    if (examDaysBySubject.exists(_.length == 0)) None else {
      val ub: DayIndex = numDays - 1
      if (!isSolution(ub)) None
      else {
        val lowerBoundByTotalPrepAndExamTime = prepTimes.sum + numSubjects - 1
        val lowerBoundByMaxOfFirstExamDays = examDaysBySubject.map(_(0)).max
        val lowerBoundIndex = lowerBoundByMaxOfFirstExamDays max lowerBoundByTotalPrepAndExamTime
        val lb: DayIndex = lowerBoundIndex
        if (isSolution(lb)) Some(lb) else {
          if (lowerBoundIndex >= numDays) None
          else if (lowerBoundIndex == numDays - 1) Some(lb)
          else Some(search(lb, ub))
        }
      }
    }
  }
}
