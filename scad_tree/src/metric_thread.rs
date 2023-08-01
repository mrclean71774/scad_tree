// MIT License
//
// Copyright (c) 2023 Michael H. Phillips
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use {
    crate::prelude::*,
    scad_tree_math::{dcos, dsin},
    std::collections::HashMap,
};

fn lerp(start: Pt3, end: Pt3, n_steps: usize, step: usize) -> Pt3 {
    start + ((end - start) / n_steps as f64 * step as f64)
}

/// Returns the dictionary for the given M size.
///
/// This function always returns a valid
/// dictionary by giving the next smallest size if the requested size is not found. If
/// a size smaller than the smallest is requested the smallest size in dict is returned.
///
/// m: The size of the thread you want dict for e.g. 6 for M6 screw threads.
///
/// return: The dictionary of thread attributes.
fn m_table_lookup(m: i32) -> HashMap<&'static str, f64> {
    let m_table = m_table();
    let mut m = m;
    if m < 2 {
        m = 2;
    }
    loop {
        if m_table.contains_key(&m) {
            break;
        }
        m -= 1;
    }
    return m_table[&m].clone();
}

/// Calculates the thread height from the given pitch.
///
/// pitch: The pitch of the threads.
///
/// return: The height of the threads.
fn thread_height_from_pitch(pitch: f64) -> f64 {
    3.0f64.sqrt() / 2.0 * pitch
}

///  Calculates the dMin of a thread based on the dMaj and pitch.
///
///  d_maj: The dMaj of the threads.
///
///  pitch: The pitch of the threads.
///
///  return: The dMin of the threads.
fn d_min_from_d_maj_pitch(d_maj: f64, pitch: f64) -> f64 {
    d_maj - 2.0 * 5.0 / 8.0 * thread_height_from_pitch(pitch)
}

/// Creates a threaded cylinder.
///
/// d_min: dMin of thread.
///
/// d_maj: dMaj of thread.
///
/// pitch: Pitch of the thread.
///
/// length: The length of the threaded rod.
///
/// segments: The number of segments in a full revolution.
///
/// lead_in_degrees: The total angle of lead in.
///
/// lead_out_degrees: The total angle of lead out.
///
/// left_hand_thread: lefty tighty?
///
/// center: Center vertically.
///
/// return: The threaded cylinder.
fn threaded_cylinder(
    d_min: f64,
    d_maj: f64,
    pitch: f64,
    length: f64,
    segments: u64,
    lead_in_degrees: f64,
    lead_out_degrees: f64,
    left_hand_thread: bool,
    center: bool,
) -> Scad {
    let lead_in = lead_in_degrees > 0.0;
    let lead_out = lead_out_degrees > 0.0;
    let thread_length = length - 0.7 * pitch;
    let n_revolutions = thread_length / pitch;
    let n_steps = (n_revolutions * segments as f64) as usize;
    let z_step = thread_length / n_steps as f64;
    let step_angle = 360.0 / segments as f64;
    let n_lead_in_steps = (segments as f64 * lead_in_degrees / 360.0 + 2.0) as usize;
    let n_lead_out_steps = (segments as f64 * lead_out_degrees / 360.0) as usize;
    let mut lead_in_step = 2;
    let mut lead_out_step = n_lead_out_steps;

    let thread_profile0 = Pt3::new(d_min / 2.0, 0.0, 3.0 / 4.0 * pitch);
    let thread_profile1 = Pt3::new(d_maj / 2.0, 0.0, 7.0 / 16.0 * pitch);
    let thread_profile2 = Pt3::new(d_min / 2.0, 0.0, 0.0);
    let thread_profile3 = Pt3::new(d_maj / 2.0, 0.0, 5.0 / 16.0 * pitch);

    let lerp_profile1 = Pt3::new(d_min / 2.0, 0.0, 7.0 / 16.0 * pitch);
    let lerp_profile3 = Pt3::new(d_min / 2.0, 0.0, 5.0 / 16.0 * pitch);

    let lead_in_start_profile0 = thread_profile0;
    let lead_in_start_profile2 = thread_profile2;
    let lead_in_start_profile1 = lerp(
        lerp_profile1,
        thread_profile1,
        n_lead_in_steps,
        lead_in_step,
    );
    let lead_in_start_profile3 = lerp(
        lerp_profile3,
        thread_profile3,
        n_lead_in_steps,
        lead_in_step,
    );
    lead_in_step += 1;

    let lead_out_end_profile1 = lerp(lerp_profile1, thread_profile1, n_lead_out_steps, 1);
    let lead_out_end_profile3 = lerp(lerp_profile3, thread_profile3, n_lead_out_steps, 1);

    let mut vertices: Vec<Pt3> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();

    // Create the starting end face
    vertices.push(lead_in_start_profile0);
    vertices.push(lead_in_start_profile1);
    vertices.push(lead_in_start_profile2);
    vertices.push(lead_in_start_profile3);

    if left_hand_thread {
        indices.append(&mut vec![2, 1, 0]);
        indices.append(&mut vec![3, 1, 2]);
    } else {
        indices.append(&mut vec![0, 1, 2]);
        indices.append(&mut vec![2, 1, 3]);
    }

    // Vertices used for the middle sections
    let mut p4;
    let mut p5;
    let mut p6;
    let mut p7;

    let lead_in_profile0 = lead_in_start_profile0;
    let mut lead_in_profile1 = lead_in_start_profile1;
    let lead_in_profile2 = lead_in_start_profile2;
    let mut lead_in_profile3 = lead_in_start_profile3;

    let lead_out_profile0 = thread_profile0;
    let mut lead_out_profile1 = thread_profile1;
    let lead_out_profile2 = thread_profile2;
    let mut lead_out_profile3 = thread_profile3;

    for step in 0..(n_steps - 1) {
        let mut angle = step_angle * (step + 1) as f64;
        if left_hand_thread {
            angle *= -1.0;
        }
        let c = dcos(angle);
        let s = dsin(angle);
        if lead_in_step < n_lead_in_steps && lead_in {
            p4 = Pt3::new(
                c * lead_in_profile0.x,
                s * lead_in_profile0.x,
                z_step * step as f64 + lead_in_profile0.z,
            );
            p5 = Pt3::new(
                c * lead_in_profile1.x,
                s * lead_in_profile1.x,
                z_step * step as f64 + lead_in_profile1.z,
            );
            p6 = Pt3::new(
                c * lead_in_profile2.x,
                s * lead_in_profile2.x,
                z_step * step as f64 + lead_in_profile2.z,
            );
            p7 = Pt3::new(
                c * lead_in_profile3.x,
                s * lead_in_profile3.x,
                z_step * step as f64 + lead_in_profile3.z,
            );

            lead_in_step += 1;
            lead_in_profile1 = lerp(
                lead_in_start_profile1,
                thread_profile1,
                n_lead_in_steps,
                lead_in_step,
            );
            lead_in_profile3 = lerp(
                lead_in_start_profile3,
                thread_profile3,
                n_lead_in_steps,
                lead_in_step,
            );
        } else if lead_out_step > 0 && step >= n_steps - n_lead_out_steps && lead_out {
            p4 = Pt3::new(
                c * lead_out_profile0.x,
                s * lead_out_profile0.x,
                z_step * step as f64 + lead_out_profile0.z,
            );
            p5 = Pt3::new(
                c * lead_out_profile1.x,
                s * lead_out_profile1.x,
                z_step * step as f64 + lead_out_profile1.z,
            );
            p6 = Pt3::new(
                c * lead_out_profile2.x,
                s * lead_out_profile2.x,
                z_step * step as f64 + lead_out_profile2.z,
            );
            p7 = Pt3::new(
                c * lead_out_profile3.x,
                s * lead_out_profile3.x,
                z_step * step as f64 + lead_out_profile3.z,
            );
            lead_out_step -= 1;
            lead_out_profile1 = lerp(
                thread_profile1,
                lead_out_end_profile1,
                n_lead_out_steps,
                n_lead_out_steps - lead_out_step,
            );
            lead_out_profile3 = lerp(
                thread_profile3,
                lead_out_end_profile3,
                n_lead_out_steps,
                n_lead_out_steps - lead_out_step,
            );
        } else {
            p4 = Pt3::new(
                c * thread_profile0.x,
                s * thread_profile0.x,
                z_step * step as f64 + thread_profile0.z,
            );
            p5 = Pt3::new(
                c * thread_profile1.x,
                s * thread_profile1.x,
                z_step * step as f64 + thread_profile1.z,
            );
            p6 = Pt3::new(
                c * thread_profile2.x,
                s * thread_profile2.x,
                z_step * step as f64 + thread_profile2.z,
            );
            p7 = Pt3::new(
                c * thread_profile3.x,
                s * thread_profile3.x,
                z_step * step as f64 + thread_profile3.z,
            );
        }

        vertices.push(p4);
        vertices.push(p5);
        vertices.push(p6);
        vertices.push(p7);

        let index_offset = step * 4;
        if left_hand_thread {
            indices.append(&mut vec![
                3 + index_offset,
                5 + index_offset,
                1 + index_offset,
            ]);
            indices.append(&mut vec![
                7 + index_offset,
                5 + index_offset,
                3 + index_offset,
            ]);
            indices.append(&mut vec![
                1 + index_offset,
                4 + index_offset,
                0 + index_offset,
            ]);
            indices.append(&mut vec![
                5 + index_offset,
                4 + index_offset,
                1 + index_offset,
            ]);
            indices.append(&mut vec![
                0 + index_offset,
                6 + index_offset,
                2 + index_offset,
            ]);
            indices.append(&mut vec![
                4 + index_offset,
                6 + index_offset,
                0 + index_offset,
            ]);
            indices.append(&mut vec![
                2 + index_offset,
                7 + index_offset,
                3 + index_offset,
            ]);
            indices.append(&mut vec![
                6 + index_offset,
                7 + index_offset,
                2 + index_offset,
            ]);
        } else {
            indices.append(&mut vec![
                1 + index_offset,
                5 + index_offset,
                3 + index_offset,
            ]);
            indices.append(&mut vec![
                3 + index_offset,
                5 + index_offset,
                7 + index_offset,
            ]);
            indices.append(&mut vec![
                0 + index_offset,
                4 + index_offset,
                1 + index_offset,
            ]);
            indices.append(&mut vec![
                1 + index_offset,
                4 + index_offset,
                5 + index_offset,
            ]);
            indices.append(&mut vec![
                2 + index_offset,
                6 + index_offset,
                0 + index_offset,
            ]);
            indices.append(&mut vec![
                0 + index_offset,
                6 + index_offset,
                4 + index_offset,
            ]);
            indices.append(&mut vec![
                3 + index_offset,
                7 + index_offset,
                2 + index_offset,
            ]);
            indices.append(&mut vec![
                2 + index_offset,
                7 + index_offset,
                6 + index_offset,
            ]);
        }
    } // end loop

    let index_offset = (n_steps - 2) * 4;
    if left_hand_thread {
        indices.append(&mut vec![
            5 + index_offset,
            7 + index_offset,
            6 + index_offset,
        ]);
        indices.append(&mut vec![
            4 + index_offset,
            5 + index_offset,
            6 + index_offset,
        ]);
    } else {
        indices.append(&mut vec![
            6 + index_offset,
            7 + index_offset,
            5 + index_offset,
        ]);
        indices.append(&mut vec![
            6 + index_offset,
            5 + index_offset,
            4 + index_offset,
        ]);
    }

    let mut faces = Faces::with_capacity(indices.len() / 3);
    for i in (0..indices.len()).step_by(3) {
        faces.push(Indices::from_indices(vec![
            indices[i] as u64,
            indices[i + 1] as u64,
            indices[i + 2] as u64,
        ]));
    }
    let convexity = (length / pitch) as u64 + 1;
    let threads = polyhedron!(Pt3s::from_pt3s(vertices), faces, convexity);

    let rod = Polyhedron::cylinder(d_min / 2.0 + 0.0001, length, segments as u64).into_scad();

    let mut result = threads + rod;

    if center {
        result = translate!([0.0, 0.0, -length / 2.0], result;);
    }
    result
}

/// Creates a threaded rod at the world origin.
///
/// m: The metric size of the rod.
///
/// length: The length of the rod in mm.
///
/// segments: The number of segments in a circle.
///
/// lead_in_degrees: Span of the lead in.
///
/// lead_out_degrees: Span of the lead out.
///
/// left_hand_thread: lefty tighty?
///
/// center: Center vertically.
///
/// return: The threaded rod.
pub fn threaded_rod(
    m: i32,
    length: f64,
    segments: u64,
    lead_in_degrees: f64,
    lead_out_degrees: f64,
    left_hand_thread: bool,
    center: bool,
) -> Scad {
    let thread_info = m_table_lookup(m);
    let pitch = thread_info["pitch"];
    let d_maj = thread_info["external_dMaj"];
    let d_min = d_min_from_d_maj_pitch(d_maj, pitch);

    threaded_cylinder(
        d_min,
        d_maj,
        pitch,
        length,
        segments,
        lead_in_degrees,
        lead_out_degrees,
        left_hand_thread,
        center,
    )
}

/// Create a hex head bolt at the world origin.
///
/// m: The metric bolt size.
///
/// length: The length of the threaded part.
///
/// head_height: The height of the hex head.
///
/// segments: The number of segments in a circle.
///
/// lead_in_degrees: The amount of degrees the tapered thread occupies.
///
/// chamfered: Whether or not to chamfer the top and bottom of the head.
///
/// left_hand_thread: lefty tighty?
///
/// center: Center vertically.
///
/// return: The hex bolt.
pub fn hex_bolt(
    m: i32,
    length: f64,
    head_height: f64,
    segments: u64,
    lead_in_degrees: f64,
    chamfered: bool,
    left_hand_thread: bool,
    center: bool,
) -> Scad {
    let thread_info = m_table_lookup(m);
    let pitch = thread_info["pitch"];
    let d_maj = thread_info["external_dMaj"];
    let head_diameter = thread_info["nut_width"];
    let d_min = d_min_from_d_maj_pitch(d_maj, pitch);

    let mut rod = threaded_cylinder(
        d_min,
        d_maj,
        pitch,
        length,
        segments,
        0.0,
        lead_in_degrees,
        left_hand_thread,
        false,
    );
    rod = translate!([0.0, 0.0, head_height], rod;);

    let mut head = Polyhedron::linear_extrude(
        &dim2::circumscribed_polygon(6, head_diameter / 2.0),
        head_height,
    )
    .into_scad();
    if chamfered {
        let chamfer_size = thread_info["chamfer_size"];
        head = head
            - Scad::external_cylinder_chamfer(
                chamfer_size,
                1.0,
                (0.25 * head_diameter * 0.25 * head_diameter
                    + 0.5 * head_diameter * 0.5 * head_diameter)
                    .sqrt(),
                head_height,
                segments,
                center,
            );
    }
    let mut bolt = rod + head;
    if center {
        bolt = translate!([0.0, 0.0, -((head_height + length) / 2.0)], bolt;);
    }
    bolt
}

/// Create a tap for making threaded holes in things.
///
/// m: The metric size of the tap.
///
/// length: The length of the tap.
///
/// segments: The number of segmentst in a circle.
///
/// left_hand_thread: lefty tighty?
///
/// center: Center vertically.
///
/// return: The tap.
pub fn tap(m: i32, length: f64, segments: u64, left_hand_thread: bool, center: bool) -> Scad {
    let thread_info = m_table_lookup(m);
    let pitch = thread_info["pitch"];
    let d_maj = thread_info["internal_dMaj"];
    let d_min = d_min_from_d_maj_pitch(d_maj, pitch);

    threaded_cylinder(
        d_min,
        d_maj,
        pitch,
        length,
        segments,
        0.0,
        0.0,
        left_hand_thread,
        center,
    )
}

/// Create a hex nut.
///
/// m: The metric size of the nut.
///
/// height: The height of the nut.
///
/// segments: The number of segments in a circle.
///
/// chamfered: Adds a chamfer to the nut.
///
/// left_hand_thread: lefty tighty?
///
/// center: Center horizontally.
///
/// return: The nut.
pub fn hex_nut(
    m: i32,
    height: f64,
    segments: u64,
    chamfered: bool,
    left_hand_thread: bool,
    center: bool,
) -> Scad {
    let thread_info = m_table_lookup(m);
    let nut_width = thread_info["nut_width"];

    let mut nut_tap = tap(m, height + 20.0, segments, left_hand_thread, center);
    nut_tap = translate!([0.0, 0.0, -10.0], nut_tap;);

    let nut_blank =
        Polyhedron::linear_extrude(&dim2::circumscribed_polygon(6, nut_width / 2.0), height)
            .into_scad();

    let mut nut = nut_blank - nut_tap;
    if chamfered {
        let chamfer_size = thread_info["chamfer_size"];
        nut = nut
            - Scad::external_cylinder_chamfer(
                chamfer_size,
                1.0,
                (0.25 * nut_width * 0.25 * nut_width + 0.5 * nut_width * 0.5 * nut_width).sqrt(),
                height,
                segments,
                center,
            );
    }

    if center {
        nut = translate!([0.0, 0.0, -height / 2.0], nut;);
    }

    nut
}

/// Returns the hashmap of iso metric thread profiles
fn m_table() -> HashMap<i32, HashMap<&'static str, f64>> {
    HashMap::from([
        (
            2,
            HashMap::from([
                ("pitch", 0.4),
                ("external_dMaj", 1.886),
                ("internal_dMaj", 2.148),
                ("nut_width", 4.0),
                ("chamfer_size", 1.45),
            ]),
        ),
        (
            3,
            HashMap::from([
                ("pitch", 0.5),
                ("external_dMaj", 2.874),
                ("internal_dMaj", 3.172),
                ("nut_width", 5.5),
                ("chamfer_size", 1.6),
            ]),
        ),
        (
            4,
            HashMap::from([
                ("pitch", 0.7),
                ("external_dMaj", 3.838),
                ("internal_dMaj", 4.219),
                ("nut_width", 7.0),
                ("chamfer_size", 1.8),
            ]),
        ),
        (
            5,
            HashMap::from([
                ("pitch", 0.8),
                ("external_dMaj", 4.826),
                ("internal_dMaj", 5.24),
                ("nut_width", 8.0),
                ("chamfer_size", 1.9),
            ]),
        ),
        (
            6,
            HashMap::from([
                ("pitch", 1.0),
                ("external_dMaj", 5.794),
                ("internal_dMaj", 6.294),
                ("nut_width", 10.0),
                ("chamfer_size", 2.1),
            ]),
        ),
        // nut_width made up for next entry
        (
            7,
            HashMap::from([
                ("pitch", 1.0),
                ("external_dMaj", 6.794),
                ("internal_dMaj", 7.294),
                ("nut_width", 13.0),
                ("chamfer_size", 2.45),
            ]),
        ),
        (
            8,
            HashMap::from([
                ("pitch", 1.25),
                ("external_dMaj", 7.76),
                ("internal_dMaj", 8.34),
                ("nut_width", 13.0),
                ("chamfer_size", 2.45),
            ]),
        ),
        // nut_width made up for next entry
        (
            9,
            HashMap::from([
                ("pitch", 1.25),
                ("external_dMaj", 8.76),
                ("internal_dMaj", 9.34),
                ("nut_width", 16.0),
                ("chamfer_size", 2.8),
            ]),
        ),
        (
            10,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 9.732),
                ("internal_dMaj", 10.396),
                ("nut_width", 16.0),
                ("chamfer_size", 2.8),
            ]),
        ),
        // nut_width made up for next entry
        (
            11,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 10.73),
                ("internal_dMaj", 11.387),
                ("nut_width", 18.0),
                ("chamfer_size", 3.0),
            ]),
        ),
        (
            12,
            HashMap::from([
                ("pitch", 1.75),
                ("external_dMaj", 11.7),
                ("internal_dMaj", 12.453),
                ("nut_width", 18.0),
                ("chamfer_size", 3.0),
            ]),
        ),
        (
            14,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 13.68),
                ("internal_dMaj", 14.501),
                ("nut_width", 21.0),
                ("chamfer_size", 3.35),
            ]),
        ),
        // nut_width made up for next entry
        (
            15,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 14.73),
                ("internal_dMaj", 15.407),
                ("nut_width", 24.0),
                ("chamfer_size", 3.7),
            ]),
        ),
        (
            16,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 15.68),
                ("internal_dMaj", 16.501),
                ("nut_width", 24.0),
                ("chamfer_size", 3.7),
            ]),
        ),
        // nut_width made up for next entry
        (
            17,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 16.73),
                ("internal_dMaj", 17.407),
                ("nut_width", 27.0),
                ("chamfer_size", 3.9),
            ]),
        ),
        (
            18,
            HashMap::from([
                ("pitch", 2.5),
                ("external_dMaj", 17.62),
                ("internal_dMaj", 18.585),
                ("nut_width", 27.0),
                ("chamfer_size", 3.9),
            ]),
        ),
        (
            20,
            HashMap::from([
                ("pitch", 2.5),
                ("external_dMaj", 19.62),
                ("internal_dMaj", 20.585),
                ("nut_width", 30.0),
                ("chamfer_size", 4.25),
            ]),
        ),
        (
            22,
            HashMap::from([
                ("pitch", 3.0),
                ("external_dMaj", 21.58),
                ("internal_dMaj", 22.677),
                ("nut_width", 34.0),
                ("chamfer_size", 4.75),
            ]),
        ),
        (
            24,
            HashMap::from([
                ("pitch", 3.0),
                ("external_dMaj", 23.58),
                ("internal_dMaj", 24.698),
                ("nut_width", 36.0),
                ("chamfer_size", 4.9),
            ]),
        ),
        // nut_width made up for next entry
        (
            25,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 24.68),
                ("internal_dMaj", 25.513),
                ("nut_width", 41.0),
                ("chamfer_size", 5.5),
            ]),
        ),
        // nut_width made up for next entry
        (
            26,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 25.73),
                ("internal_dMaj", 26.417),
                ("nut_width", 41.0),
                ("chamfer_size", 5.5),
            ]),
        ),
        (
            27,
            HashMap::from([
                ("pitch", 3.0),
                ("external_dMaj", 26.58),
                ("internal_dMaj", 27.698),
                ("nut_width", 41.0),
                ("chamfer_size", 5.5),
            ]),
        ),
        // nut_width made up for next entry
        (
            28,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 27.68),
                ("internal_dMaj", 28.513),
                ("nut_width", 46.0),
                ("chamfer_size", 6.0),
            ]),
        ),
        (
            30,
            HashMap::from([
                ("pitch", 3.5),
                ("external_dMaj", 29.52),
                ("internal_dMaj", 30.785),
                ("nut_width", 46.0),
                ("chamfer_size", 6.0),
            ]),
        ),
        // nut_width made up for next entry
        (
            32,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 31.68),
                ("internal_dMaj", 32.513),
                ("nut_width", 49.0),
                ("chamfer_size", 6.4),
            ]),
        ),
        (
            33,
            HashMap::from([
                ("pitch", 3.5),
                ("external_dMaj", 32.54),
                ("internal_dMaj", 33.785),
                ("nut_width", 49.0),
                ("chamfer_size", 6.4),
            ]),
        ),
        // nut_width made up for next entry
        (
            35,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 34.73),
                ("internal_dMaj", 35.416),
                ("nut_width", 55.0),
                ("chamfer_size", 7.0),
            ]),
        ),
        (
            36,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 35.47),
                ("internal_dMaj", 36.877),
                ("nut_width", 55.0),
                ("chamfer_size", 7.0),
            ]),
        ),
        // nut_width made up for next entry
        (
            38,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 37.73),
                ("internal_dMaj", 38.417),
                ("nut_width", 60.0),
                ("chamfer_size", 7.5),
            ]),
        ),
        (
            39,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 38.47),
                ("internal_dMaj", 39.877),
                ("nut_width", 60.0),
                ("chamfer_size", 7.5),
            ]),
        ),
        // nut_width made up for next entry
        (
            40,
            HashMap::from([
                ("pitch", 3.0),
                ("external_dMaj", 39.58),
                ("internal_dMaj", 40.698),
                ("nut_width", 65.0),
                ("chamfer_size", 8.2),
            ]),
        ),
        (
            42,
            HashMap::from([
                ("pitch", 4.5),
                ("external_dMaj", 41.44),
                ("internal_dMaj", 42.965),
                ("nut_width", 65.0),
                ("chamfer_size", 8.2),
            ]),
        ),
        (
            45,
            HashMap::from([
                ("pitch", 4.5),
                ("external_dMaj", 44.44),
                ("internal_dMaj", 45.965),
                ("nut_width", 70.0),
                ("chamfer_size", 8.75),
            ]),
        ),
        (
            48,
            HashMap::from([
                ("pitch", 5.0),
                ("external_dMaj", 47.4),
                ("internal_dMaj", 49.057),
                ("nut_width", 75.0),
                ("chamfer_size", 9.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            50,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 49.47),
                ("internal_dMaj", 50.892),
                ("nut_width", 80.0),
                ("chamfer_size", 9.5),
            ]),
        ),
        (
            52,
            HashMap::from([
                ("pitch", 5.0),
                ("external_dMaj", 51.4),
                ("internal_dMaj", 53.037),
                ("nut_width", 80.0),
                ("chamfer_size", 9.5),
            ]),
        ),
        // nut_width made up for next entry
        (
            55,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 54.47),
                ("internal_dMaj", 55.892),
                ("nut_width", 85.0),
                ("chamfer_size", 10.25),
            ]),
        ),
        (
            56,
            HashMap::from([
                ("pitch", 5.5),
                ("external_dMaj", 55.37),
                ("internal_dMaj", 57.149),
                ("nut_width", 85.0),
                ("chamfer_size", 10.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            58,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 57.47),
                ("internal_dMaj", 58.892),
                ("nut_width", 90.0),
                ("chamfer_size", 10.75),
            ]),
        ),
        (
            60,
            HashMap::from([
                ("pitch", 5.5),
                ("external_dMaj", 59.37),
                ("internal_dMaj", 61.149),
                ("nut_width", 90.0),
                ("chamfer_size", 10.75),
            ]),
        ),
        // nut_width made up for next entry
        (
            62,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 61.47),
                ("internal_dMaj", 62.892),
                ("nut_width", 95.0),
                ("chamfer_size", 11.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            63,
            HashMap::from([
                ("pitch", 1.5),
                ("external_dMaj", 62.73),
                ("internal_dMaj", 63.429),
                ("nut_width", 95.0),
                ("chamfer_size", 11.25),
            ]),
        ),
        (
            64,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 63.32),
                ("internal_dMaj", 65.421),
                ("nut_width", 95.0),
                ("chamfer_size", 11.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            65,
            HashMap::from([
                ("pitch", 4.0),
                ("external_dMaj", 64.47),
                ("internal_dMaj", 65.892),
                ("nut_width", 100.0),
                ("chamfer_size", 11.75),
            ]),
        ),
        // nut_width made up for next entry
        (
            68,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 67.32),
                ("internal_dMaj", 69.241),
                ("nut_width", 100.0),
                ("chamfer_size", 11.75),
            ]),
        ),
        // nut_width made up for next entry
        (
            70,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 69.32),
                ("internal_dMaj", 71.241),
                ("nut_width", 100.0),
                ("chamfer_size", 11.75),
            ]),
        ),
        // nut_width made up for next entry
        (
            72,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 71.32),
                ("internal_dMaj", 73.241),
                ("nut_width", 110.0),
                ("chamfer_size", 13.0),
            ]),
        ),
        // nut_width made up for next entry
        (
            75,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 74.32),
                ("internal_dMaj", 76.241),
                ("nut_width", 110.0),
                ("chamfer_size", 13.0),
            ]),
        ),
        // nut_width made up for next entry
        (
            76,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 75.32),
                ("internal_dMaj", 77.241),
                ("nut_width", 110.0),
                ("chamfer_size", 13.0),
            ]),
        ),
        // nut_width made up for next entry
        (
            78,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 77.68),
                ("internal_dMaj", 78.525),
                ("nut_width", 120.0),
                ("chamfer_size", 14.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            80,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 79.32),
                ("internal_dMaj", 81.241),
                ("nut_width", 120.0),
                ("chamfer_size", 14.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            82,
            HashMap::from([
                ("pitch", 2.0),
                ("external_dMaj", 81.68),
                ("internal_dMaj", 82.525),
                ("nut_width", 120.0),
                ("chamfer_size", 14.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            85,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 84.32),
                ("internal_dMaj", 86.241),
                ("nut_width", 130.0),
                ("chamfer_size", 15.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            90,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 89.32),
                ("internal_dMaj", 91.241),
                ("nut_width", 130.0),
                ("chamfer_size", 15.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            95,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 94.32),
                ("internal_dMaj", 96.266),
                ("nut_width", 130.0),
                ("chamfer_size", 15.25),
            ]),
        ),
        // nut_width made up for next entry
        (
            100,
            HashMap::from([
                ("pitch", 6.0),
                ("external_dMaj", 99.32),
                ("internal_dMaj", 101.27),
                ("nut_width", 140.0),
                ("chamfer_size", 16.5),
            ]),
        ),
    ])
}
