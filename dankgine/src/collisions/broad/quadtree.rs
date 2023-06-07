use crate::geometry::{verlet::VerletObject, rectangle::Rectangle};

#[derive(Debug)]
pub struct QuadTree {
    pub bounds: Rectangle,
    pub content: Vec<VerletObject>,
    pub north_west: Option<Box<QuadTree>>,
    pub north_east: Option<Box<QuadTree>>,
    pub south_west: Option<Box<QuadTree>>,
    pub south_east: Option<Box<QuadTree>>,
    pub limit: usize,
    pub depth: usize,
    pub max_depth: usize,
}

impl QuadTree {
    pub fn new(bounds: Rectangle, limit: usize, depth: usize, max_depth: usize) -> QuadTree {
        QuadTree {
            bounds: bounds,
            content: Vec::new(), 
            north_west: None, 
            north_east: None, 
            south_west: None, 
            south_east: None,
            limit: limit,
            depth: depth,
            max_depth: max_depth,
        }
    }

    pub fn insert(&mut self, object: VerletObject) {
        if !self.bounds.contains(object.current_position) {
            return;
        }

        if self.north_west.is_some() { // has children
            self.north_west.as_mut().unwrap().insert(object);
            self.north_east.as_mut().unwrap().insert(object);
            self.south_west.as_mut().unwrap().insert(object);
            self.south_east.as_mut().unwrap().insert(object);
        } else { // is leaf node
            self.content.push(object);
            if self.content.len() >= self.limit && self.depth <= self.max_depth {
                self.split();
            }
        }
    }

    fn split(&mut self) {
        let x = self.bounds.position.x;
        let y = self.bounds.position.y;
        let new_width = self.bounds.width / 2.0;
        let new_height = self.bounds.height / 2.0;

        let north_west_bounds = Rectangle::new(x, y, new_width, new_height);
        let north_east_bounds = Rectangle::new(x + new_width, y, new_width, new_height);
        let south_west_bounds = Rectangle::new(x, y + new_height, new_width, new_height);
        let south_east_bounds = Rectangle::new(x + new_width, y + new_height, new_width, new_height);

        self.north_west = Some(Box::new(QuadTree::new(north_west_bounds, self.limit, self.depth + 1, self.max_depth)));
        self.north_east = Some(Box::new(QuadTree::new(north_east_bounds, self.limit, self.depth + 1, self.max_depth)));
        self.south_west = Some(Box::new(QuadTree::new(south_west_bounds, self.limit, self.depth + 1, self.max_depth)));
        self.south_east = Some(Box::new(QuadTree::new(south_east_bounds, self.limit, self.depth + 1, self.max_depth)));

        for object in &self.content {
            self.north_west.as_mut().unwrap().insert(*object);
            self.north_east.as_mut().unwrap().insert(*object);
            self.south_west.as_mut().unwrap().insert(*object);
            self.south_east.as_mut().unwrap().insert(*object);
        }

        self.content.clear();
    }

    pub fn query(&mut self, range: &Rectangle) -> Vec<VerletObject>{
        if !self.bounds.intersects(range) {
            return Vec::new();
        }

        let mut result: Vec<VerletObject> = Vec::new();

        for object in self.content.iter_mut() {
            if range.contains(object.current_position) {
               result.push(*object);
            }
        }
        
        if self.north_west.is_some() {
            let mut nw_res = self.north_west.as_mut().unwrap().query(range);
            if !nw_res.is_empty() {
                result.append(&mut nw_res);
            }
            let mut ne_res = self.north_east.as_mut().unwrap().query(range);
            if !ne_res.is_empty() {
                result.append(&mut ne_res);
            }
            let mut sw_res = self.south_west.as_mut().unwrap().query(range);
            if !sw_res.is_empty() {
                result.append(&mut sw_res);
            }
            let mut se_res = self.south_east.as_mut().unwrap().query(range);
            if !se_res.is_empty() {
                result.append(&mut se_res);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{collisions::broad::quadtree::QuadTree, geometry::{rectangle::Rectangle, verlet::VerletObject, vector::Vec2}};

    fn create_basic_quadtree() -> QuadTree {
        QuadTree::new(Rectangle::new(0.0, 0.0, 100.0, 100.0), 3, 0, 10)
    }

    fn has_any_children(q_tree: &QuadTree) -> bool {
        q_tree.north_west.is_some() || q_tree.north_east.is_some() || q_tree.south_west.is_some() || q_tree.south_east.is_some()
    }

    fn brute_force_query(objects: &Vec<VerletObject>, range: Rectangle) -> Vec<&VerletObject> {
        let mut result = Vec::new();

        for object in objects {
            if range.contains(object.current_position) {
                result.push(object);
            }
        }

        return result;
    }

    fn same_query_results(result1: Vec<VerletObject>, result2: Vec<&VerletObject>) -> bool{
        return result1.iter().all(|item| result2.contains(&item)) &&
            result2.iter().all(|item| result1.contains(item))
    }

    #[test]
    fn out_of_range() {
        let mut q_tree = create_basic_quadtree();
        
        let obj1 = VerletObject::new(Vec2::new(200.0, 200.0), 10.0);
        let obj2 = VerletObject::new(Vec2::new(200.0, 200.0), 1000.0);
        let obj3 = VerletObject::new(Vec2::new(-100.0, 0.0), 10.0);
        let obj4 = VerletObject::new(Vec2::new(-1.0, 0.0), 20.0);

        q_tree.insert(obj1);
        q_tree.insert(obj2);
        q_tree.insert(obj3);
        q_tree.insert(obj4);

        assert!(q_tree.content.is_empty());
        assert!(q_tree.depth == 0);
    }

    #[test]
    fn tree_structure() {
        let mut q_tree = create_basic_quadtree();
        let obj1 = VerletObject::new(Vec2::new(10.0, 10.0), 2.0);
        let obj2 = VerletObject::new(Vec2::new(12.0, 8.0), 3.0);
        let obj3 = VerletObject::new(Vec2::new(80.0, 74.0), 1.0);
        let obj4 = VerletObject::new(Vec2::new(26.0, 30.0), 2.0);
        let obj5 = VerletObject::new(Vec2::new(2.0, 3.0), 10.0);

        q_tree.insert(obj1);

        assert!(q_tree.content.len() == 1);
        assert!(!has_any_children(&q_tree));
        assert!(q_tree.depth == 0);

        q_tree.insert(obj2);

        assert!(q_tree.content.len() == 2);
        assert!(!has_any_children(&q_tree));

        q_tree.insert(obj3);

        assert!(q_tree.content.is_empty());
        assert!(has_any_children(&q_tree));
        assert!(q_tree.north_west.as_ref().unwrap().content.len() == 2);
        assert!(q_tree.north_east.as_ref().unwrap().content.is_empty());
        assert!(q_tree.south_west.as_ref().unwrap().content.is_empty());
        assert!(q_tree.south_east.as_ref().unwrap().content.len() == 1);
        assert!(!has_any_children(q_tree.north_west.as_ref().unwrap()));
        assert!(q_tree.north_west.as_ref().unwrap().depth == 1);

        q_tree.insert(obj4);

        assert!(has_any_children(q_tree.north_west.as_ref().unwrap()));
        assert!(!has_any_children(q_tree.north_east.as_ref().unwrap()));
        assert!(!has_any_children(q_tree.south_west.as_ref().unwrap()));
        assert!(!has_any_children(q_tree.south_east.as_ref().unwrap()));

        assert!(q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().content.len() == 2);
        assert!(q_tree.north_west.as_ref().unwrap().south_east.as_ref().unwrap().content.len() == 1);
        assert!(q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().depth == 2);
       
        q_tree.insert(obj5);

        assert!(has_any_children(q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap()));
        assert!(q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().content.is_empty());
        assert!(has_any_children(q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().north_west.as_ref().unwrap()));

        let leaf_north_west = q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().north_west.as_ref().unwrap().north_west.as_ref().unwrap();
        let leaf_south_east = q_tree.north_west.as_ref().unwrap().north_west.as_ref().unwrap().north_west.as_ref().unwrap().south_east.as_ref().unwrap();
        assert!(leaf_north_west.content.len() == 1);
        assert!(leaf_south_east.content.len() == 2);

        assert!(leaf_north_west.content.first().unwrap() == &obj5);
        assert!(leaf_south_east.content.contains(&obj1) && leaf_south_east.content.contains(&obj2));
        assert!(q_tree.south_east.as_ref().unwrap().content.contains(&obj3));
        assert!(q_tree.north_west.as_ref().unwrap().south_east.as_ref().unwrap().content.contains(&obj4));
    }

    #[test]
    fn same_point_inserts() {
        let mut q_tree = create_basic_quadtree();
        let obj1 = VerletObject::new(Vec2::new(10.0, 10.0), 2.0);

        for _ in 0..q_tree.limit {
            q_tree.insert(obj1); //if the tree does not cap at a certain depth then this will cause a stack overflow
        }
    }

    #[test]
    fn query_range() {
        let mut q_tree = create_basic_quadtree();
        let objects = [ 
            VerletObject::new(Vec2::new(91.44708336564145, 2.7679874904863455), 27.200550827881575),
            VerletObject::new(Vec2::new(0.4231728170873694, 68.92729562434513), 61.32362243721943),
            VerletObject::new(Vec2::new(63.97594512594329, 43.26993791110982), 11.724898328050127),
            VerletObject::new(Vec2::new(34.52942370169565, 40.214313288944446), 75.75191737536291),
            VerletObject::new(Vec2::new(97.10877889136324, 84.50222061983781), 75.98028073452289),
            VerletObject::new(Vec2::new(59.96339967705999, 97.92134839991138), 77.45559171295933),
            VerletObject::new(Vec2::new(85.41500742509413, 14.93964749990302), 1.4799396803709675),
            VerletObject::new(Vec2::new(6.310127910298902, 58.87737284534189), 83.3043605691469),
            VerletObject::new(Vec2::new(73.84981599633038, 5.583568091653701), 77.23284988971932),
            VerletObject::new(Vec2::new(60.09534941699446, 98.65349659701516), 29.954867541934572),
            VerletObject::new(Vec2::new(67.44854751334886, 53.592298609441194), 47.90077630149081),
            VerletObject::new(Vec2::new(91.86543744666787, 90.44701411890684), 61.261234034363945),
            VerletObject::new(Vec2::new(26.85872423287805, 3.541819558112591), 90.95995475089533),
            VerletObject::new(Vec2::new(77.74619509367263, 21.796425292886525), 66.429514840552),
            VerletObject::new(Vec2::new(46.12284001658491, 59.56180978900407), 8.221987627815409),
            VerletObject::new(Vec2::new(39.74608379835407, 94.60636303195302), 35.24488964815562),
            VerletObject::new(Vec2::new(41.499167848796816, 67.79796500403901), 17.263411835419017),
            VerletObject::new(Vec2::new(41.63377392651228, 55.02410521508996), 62.191861909936954),
            VerletObject::new(Vec2::new(29.757985869030733, 20.293675638487073), 6.0008360566227825),
            VerletObject::new(Vec2::new(99.67956156835331, 8.123923117838693), 38.85387355568501),
        ];

        let ranges = [ 
            Rectangle::new(79.52492726904161, 40.92340673809842, 42.56887199138863, 46.83846854361866),
            Rectangle::new(88.92005453209319, 6.514332730961625, 9.371704431965366, 32.78318055101901),
            Rectangle::new(73.08144945732802, 32.64517375442233, 16.151428947414225, 35.3194797369011),
            Rectangle::new(47.43672172673532, 24.965033593495754, 39.191936711407514, 24.325305399144735),
            Rectangle::new(14.668903817967372, 89.00321601700259, 11.627063460567499, 16.797429951405917),
            Rectangle::new(44.027848623826294, 66.74000109371016, 34.37710524816067, 4.897097374977855),
            Rectangle::new(36.118949213102326, 25.875079883723863, 20.021156192096925, 39.99137217580208),
            Rectangle::new(63.57677307549439, 20.83752678121926, 2.5650545267967395, 29.447806624637774),
            Rectangle::new(13.341088400156508, 83.92703384744193, 31.435263383695865, 49.30047682155257),
            Rectangle::new(49.33739607117258, 22.795749211596352, 23.073902967860015, 27.060406713422847),
            Rectangle::new(77.9811882656527, 71.91652945894313, 17.269206521912693, 4.009196038962171),
            Rectangle::new(29.754356690670836, 37.21625431357949, 19.90200937946448, 23.16399544511055),
            Rectangle::new(49.36175004816692, 68.5961622093949, 40.674935883168374, 4.832146126552416),
            Rectangle::new(7.490195306922742, 41.584798009468656, 33.990616182199076, 38.646727757201525),
            Rectangle::new(70.16571706822833, 70.28904769431125, 6.374962319321864, 4.9314815210803165),
            Rectangle::new(7.2865493560331895, 4.083402993746277, 18.406434326418577, 8.220632203978152),
            Rectangle::new(82.21693259434797, 16.919975008055776, 14.198433570924685, 40.061875737535836),
            Rectangle::new(80.55019341919719, 95.59959861212064, 42.6324243459389, 48.91764539904758),
            Rectangle::new(79.75577443674733, 75.75937076901351, 39.22755158097851, 6.35672841001772),
            Rectangle::new(28.942909699692464, 61.61963450264252, 18.8019042017462, 3.732812980882083),
            Rectangle::new(98.99738288157907, 13.01510935876664, 17.849520531534925, 13.443700100007117),
            Rectangle::new(32.87100685954982, 82.67314283439585, 45.071088847166585, 39.70742882203845),
            Rectangle::new(15.454057641676577, 42.053074458695264, 37.70213461040846, 20.686309660530544),
            Rectangle::new(34.60534721677158, 32.801547097018016, 15.816406403663485, 23.338337493195873),
            Rectangle::new(35.645840978073196, 3.191267415971666, 28.568953116836372, 11.751968311808248),
            Rectangle::new(62.993979815909526, 53.61781652935409, 2.3492161927005983, 0.10780741727932641),
            Rectangle::new(87.21747020536507, 83.60355826382772, 24.48137105282422, 47.72856295338023),
            Rectangle::new(31.39938533351416, 81.7385991204326, 15.70875886613089, 42.24304955895136),
            Rectangle::new(19.794849623357536, 90.91384414791757, 38.950840300919, 29.99143197505013),
            Rectangle::new(6.166098709104095, 97.78707318234412, 10.98104769262589, 45.65826394891498),
            Rectangle::new(35.402244879656934, 94.84176245692186, 49.362802366198856, 18.77862870239648),
            Rectangle::new(84.9287077519753, 17.04995797125619, 17.18483957358642, 22.52458448821152),
            Rectangle::new(78.91636504853534, 88.61458139137885, 30.95636517954056, 38.833644652664425),
            Rectangle::new(22.144973786469645, 15.386324081372749, 44.46163328256733, 32.52604341070988),
            Rectangle::new(11.677227583332783, 47.25755541052259, 11.439694082049579, 33.85903175853166),
            Rectangle::new(17.541956197445163, 98.70905311358989, 47.14636725794552, 47.97730055932331),
            Rectangle::new(86.38686751124457, 75.07206193739768, 15.168624204510039, 49.75199967997548),
            Rectangle::new(87.80061991818695, 17.323648969946028, 19.197385068208906, 47.40911205234808),
            Rectangle::new(83.85837933448836, 82.28310862331783, 44.48852985290929, 26.38311230804303),
            Rectangle::new(27.12154664745301, 43.89802357803809, 2.6024553399647488, 47.37897865162128),
            Rectangle::new(68.11595322854555, 63.66519716403487, 9.064017284604075, 27.771069728624255),
            Rectangle::new(23.066530761106918, 93.87729213440821, 38.77509866657003, 2.1559026025796846),
            Rectangle::new(8.374910554396763, 50.782640683728644, 47.31589267857795, 4.033440267170352),
            Rectangle::new(15.343790594900742, 88.56557314884358, 42.106067933016654, 10.405070869301325),
            Rectangle::new(60.94896363625726, 17.44945325488181, 28.60144116650699, 19.66224526984418),
            Rectangle::new(44.36114895406784, 93.68688590919393, 12.728035213603995, 14.559903035910915),
            Rectangle::new(94.25701097676145, 60.59040246159309, 19.350581425981893, 34.06808385038864),
            Rectangle::new(86.2807336424321, 54.37100539500448, 26.418299913689424, 26.082250568145593),
            Rectangle::new(69.4273100562833, 43.426412020056546, 41.49953546342006, 27.236587851247585),
            Rectangle::new(26.47608690100396, 49.671547070956535, 11.957852889415266, 17.452756355945766),
        ];

        for object in objects {
            q_tree.insert(object);
        }

        let objects_vec = Vec::from(objects);

        for range in ranges {
            let q_tree_query_result = q_tree.query(&range);
            let brute_force_query_result = brute_force_query(&objects_vec, range);

            assert!(same_query_results(q_tree_query_result, brute_force_query_result))
        } 
    }
}